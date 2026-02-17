use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::timeout;


mod sdk {
    use super::*;
    use std::collections::VecDeque;
    use tokio::sync::Notify;

    pub struct Sender<T> {
        queue: Arc<Mutex<VecDeque<T>>>,
        cap: usize,
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
                Sender { queue: queue.clone(), cap, notify: notify.clone() },
                Receiver { queue, notify },
            )
        }
        pub async fn send(&self, item: T) {
            loop {
                let mut q = self.queue.lock().await;
                if q.len() < self.cap {
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
}

#[tokio::test]
async fn fifo_ordering() {
    let (tx, rx) = sdk::Sender::new(10);
    tx.send(1).await;
    tx.send(2).await;
    tx.send(3).await;
    assert_eq!(rx.recv().await, 1);
    assert_eq!(rx.recv().await, 2);
    assert_eq!(rx.recv().await, 3);
}

#[tokio::test]
async fn blocking_when_full() {
    let (tx, rx) = sdk::Sender::new(2);
    tx.send(10).await;
    tx.send(20).await;
    // Third send should block until a recv happens
    let send_fut = tx.send(30);
    let send_blocked = timeout(Duration::from_millis(100), send_fut);
    assert!(send_blocked.await.is_err(), "send should block when full");
    // Now recv one, send should complete
    let recv_fut = rx.recv();
    let val = timeout(Duration::from_millis(100), recv_fut).await.unwrap();
    assert_eq!(val, 10);
}

// #[tokio::test]
// async fn capacity_increase_unblocks_sender() {
//     let (tx, rx) = sdk::Sender::new(1);
//     tx.send(1).await;
//     // Simulate capacity increase by replacing cap (for test only)
//     let tx = Arc::new(tx);
//     let tx2 = tx.clone();
//     tokio::spawn(async move {
//         tokio::time::sleep(Duration::from_millis(100)).await;
//         let mut tx_mut = Arc::get_mut(&mut tx2.clone()).unwrap();
//         tx_mut.cap = 2;
//     });
//     let send_fut = tx.send(2);
//     let _ = timeout(Duration::from_millis(300), send_fut).await.expect("send should unblock after cap increase");
//     assert_eq!(rx.recv().await, 1);
//     assert_eq!(rx.recv().await, 2);
// }

#[tokio::test]
async fn multiple_producers_single_consumer() {
    let (tx, rx) = sdk::Sender::new(5);
    let tx1 = Arc::new(tx);
    let tx2 = tx1.clone();
    let h1 = tokio::spawn(async move {
        for i in 0..5 {
            tx1.send(i).await;
        }
    });
    let h2 = tokio::spawn(async move {
        for i in 5..10 {
            tx2.send(i).await;
        }
    });
    let mut results = Vec::new();
    for _ in 0..10 {
        results.push(rx.recv().await);
    }
    h1.await.unwrap();
    h2.await.unwrap();
    results.sort();
    assert_eq!(results, (0..10).collect::<Vec<_>>());
}
