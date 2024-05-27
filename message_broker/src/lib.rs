use async_trait::async_trait;
use crate::error::MBrokerResult;


pub mod error;
pub mod rabbitmq;
pub mod tests;

#[async_trait]
pub trait Publisher<M, R = MBrokerResult<()>>: Send + Sync
    where
        M: Send + Sync,
{
    async fn publish(&self, message: M) -> R
        where
            M: 'async_trait;
}

#[async_trait]
pub trait Receiver<M, R = MBrokerResult<M>>: Send + Sync {
    async fn receive(&self) -> R;
}

#[async_trait]
pub trait Subscriber<C, O> {
    async fn init(conn: C, options: &O) -> MBrokerResult<Self>
        where
            Self: Sized;
    async fn close(&self) -> MBrokerResult<()>;
}
