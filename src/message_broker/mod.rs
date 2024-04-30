use async_trait::async_trait;

use crate::message_broker::error::MBrokerResult;

pub mod connection;
pub mod error;
pub mod rabbitmq;

#[async_trait]
pub trait Publisher<M>: Send + Sync
where
    M: Send + Sync,
{
    async fn publish(&self, message: M) -> MBrokerResult<()>
    where
        M: 'async_trait;
}

#[async_trait]
pub trait Receiver<M>: Send + Sync {
    async fn receive(&self) -> MBrokerResult<M>;
}

#[async_trait]
pub trait Subscriber<C, O> {
    async fn init(conn: C, options: O) -> MBrokerResult<Self>
    where
        Self: Sized;
    async fn close(&self) -> MBrokerResult<()>;
}
