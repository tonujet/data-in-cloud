use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;
use crate::error::MBrokerResult;
use crate::{Publisher, Receiver};


pub struct ReceiverMock<M: Send + Sync> {
    queue: Arc<Mutex<Vec<M>>>,
}

#[async_trait]
impl<M> Receiver<M> for ReceiverMock<M>
where
    M: Send + Sync,
{
    async fn receive(&self) -> MBrokerResult<M> {
        Ok(self.queue.lock().await.remove(0))
    }
}

impl<M> ReceiverMock<M>
where
    M: Send + Sync,
{
    pub fn new(queue: Arc<Mutex<Vec<M>>>) -> Self {
        Self { queue }
    }
}

pub struct PublisherMock<M, T, R = MBrokerResult<T>> {
    receiver: Arc<dyn Receiver<T, R>>,
    queue: Arc<Mutex<Vec<M>>>,
}

impl<M, T, R> PublisherMock<M, T, R> {
    pub fn new(queue: Arc<Mutex<Vec<M>>>, receiver: Arc<dyn Receiver<T, R>>) -> Self {
        Self { receiver, queue }
    }
}

#[async_trait]
impl<M, T, R> Publisher<M> for PublisherMock<M, T, R>
where
    M: Send + Sync,
{
    async fn publish(&self, message: M) -> MBrokerResult<()> {
        self.queue.lock().await.push(message);
        self.receiver.receive().await;
        Ok(())
    }
}
