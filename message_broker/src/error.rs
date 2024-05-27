use thiserror::Error;

pub type MBrokerResult<T> = Result<T, MBrokerError>;

#[derive(Debug, Error)]
pub enum MBrokerError {
    #[error(transparent)]
    RabbitMQ(#[from] amqprs::error::Error),
    
    #[error("Can't read message. {0}")]
    CantReadMessage(String),
}
