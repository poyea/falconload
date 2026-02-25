use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;

use falconload::sdk;

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

#[tokio::test]
async fn capacity_increase_unblocks_sender() {
    let (tx, rx) = sdk::Sender::new(1);
    tx.send(1).await;
    // Simulate capacity increase by calling set_capacity from another task
    let tx = Arc::new(tx);
    let tx2 = tx.clone();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(100)).await;
        tx2.set_capacity(2);
    });
    let send_fut = tx.send(2);
    let _ = timeout(Duration::from_millis(300), send_fut)
        .await
        .expect("send should unblock after cap increase");
    assert_eq!(rx.recv().await, 1);
    assert_eq!(rx.recv().await, 2);
}

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
