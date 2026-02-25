use std::collections::VecDeque;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use tokio::sync::{Mutex, Notify};

/// Simple in-memory bounded async queue with runtime-adjustable capacity.
pub struct Sender<T> {
    queue: Arc<Mutex<VecDeque<T>>>,
    cap: Arc<AtomicUsize>,
    notify: Arc<Notify>,
}

pub struct Receiver<T> {
    queue: Arc<Mutex<VecDeque<T>>>,
    notify: Arc<Notify>,
}

impl<T> Sender<T> {
    pub fn new(cap: usize) -> (Self, Receiver<T>) {
        let queue = Arc::new(Mutex::new(VecDeque::with_capacity(cap)));
        let notify = Arc::new(Notify::new());
        (
            Sender {
                queue: queue.clone(),
                cap: Arc::new(AtomicUsize::new(cap)),
                notify: notify.clone(),
            },
            Receiver { queue, notify },
        )
    }

    /// Set a new capacity at runtime. Notifies waiting tasks.
    pub fn set_capacity(&self, new_cap: usize) {
        self.cap.store(new_cap, Ordering::SeqCst);
        self.notify.notify_one();
    }

    /// Send an item, waiting while the queue is full.
    pub async fn send(&self, item: T) {
        loop {
            let mut q = self.queue.lock().await;
            let cap = self.cap.load(Ordering::SeqCst);
            if q.len() < cap {
                q.push_back(item);
                self.notify.notify_one();
                return;
            }
            drop(q);
            self.notify.notified().await;
        }
    }
}

impl<T> Receiver<T> {
    /// Receive an item, waiting if empty.
    pub async fn recv(&self) -> T {
        loop {
            let mut q = self.queue.lock().await;
            if let Some(item) = q.pop_front() {
                self.notify.notify_one();
                return item;
            }
            drop(q);
            self.notify.notified().await;
        }
    }
}
