use amqprs::BasicProperties;
use amqprs::callbacks::DefaultChannelCallback;
use amqprs::channel::{
    BasicAckArguments, BasicConsumeArguments, BasicPublishArguments, Channel, ConsumerMessage,
    QueueDeclareArguments,
};
use amqprs::connection::Connection;
use async_trait::async_trait;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::message_broker::error::{MBrokerError, MBrokerResult};

use super::{Publisher, Receiver, Subscriber};

pub struct RabbitMQOptions {
    pub queue_name: &'static str,
    pub durable: bool,
}

pub struct RabbitMQPublisher {
    channel: Mutex<Option<Channel>>,
    queue_name: String,
}

#[async_trait]
pub trait RabbitMQSubscriber: Subscriber<Connection, RabbitMQOptions> {
    async fn get_channel(&self) -> Channel;
    async fn close(&self) -> MBrokerResult<()> {
        Ok(self.get_channel().await.close().await?)
    }

    async fn init(conn: Connection, options: &RabbitMQOptions) -> MBrokerResult<(Channel, String)>
    where
        Self: Sized,
    {
        let ch = conn.open_channel(None).await?;
        ch.register_callback(DefaultChannelCallback).await?;

        let q_args = QueueDeclareArguments::default()
            .queue(options.queue_name.to_string())
            .durable(options.durable)
            .finish();

        let (queue_name, _, _) = ch.queue_declare(q_args).await?.unwrap();
        Ok((ch, queue_name))
    }
}

#[async_trait]
impl Subscriber<Connection, RabbitMQOptions> for RabbitMQPublisher {
    async fn init(conn: Connection, options: &RabbitMQOptions) -> MBrokerResult<Self> {
        let params = <Self as RabbitMQSubscriber>::init(conn, options).await?;
        Ok(Self {
            channel: Mutex::new(Some(params.0)),
            queue_name: params.1,
        })
    }

    async fn close(&self) -> MBrokerResult<()> {
        RabbitMQSubscriber::close(self).await
    }
}

#[async_trait]
impl RabbitMQSubscriber for RabbitMQPublisher {
    async fn get_channel(&self) -> Channel {
        self.channel.lock().await.take().unwrap()
    }
}

#[async_trait]
impl<M> Publisher<M> for RabbitMQPublisher
where
    M: Into<Vec<u8>> + Send + Sync,
{
    async fn publish(&self, message: M) -> MBrokerResult<()>
    where
        M: 'async_trait,
    {
        let lock = self.channel.lock().await;
        let ch = lock.as_ref().unwrap();
        let publish_args = BasicPublishArguments::new("", &self.queue_name);
        let props = BasicProperties::default().with_delivery_mode(2).finish();
        ch.basic_publish(props, message.into(), publish_args)
            .await?;

        println!(" [x] Sent");
        Ok(())
    }
}

pub struct RabbitMQReceiver {
    channel: Mutex<Option<Channel>>,
    receiver: Mutex<UnboundedReceiver<ConsumerMessage>>,
    _queue_name: String,
}

#[async_trait]
impl RabbitMQSubscriber for RabbitMQReceiver {
    async fn get_channel(&self) -> Channel {
        self.channel.lock().await.take().unwrap()
    }
}

#[async_trait]
impl Subscriber<Connection, RabbitMQOptions> for RabbitMQReceiver {
    async fn init(conn: Connection, options: &RabbitMQOptions) -> MBrokerResult<Self>
    where 
        Self: Sized,
    {
        let (ch, queue_name) = <Self as RabbitMQSubscriber>::init(conn, options).await?;
        let consume_tag = format!("{queue_name}_receiver_{}", Uuid::new_v4());

        let consumer_args = BasicConsumeArguments::new(&queue_name, &consume_tag);
        let (_ctag, rx) = ch.basic_consume_rx(consumer_args).await.unwrap();

        Ok(Self {
            channel: Mutex::new(Some(ch)),
            queue_name,
            receiver: Mutex::new(rx),
        })
    }

    async fn close(&self) -> MBrokerResult<()> {
        RabbitMQSubscriber::close(self).await
    }
}

#[async_trait]
impl<M> Receiver<M> for RabbitMQReceiver
where
    M: From<Vec<u8>>,
{
    async fn receive(&self) -> MBrokerResult<M> {
        let lock = self.channel.lock().await;
        let ch = lock.as_ref().unwrap();
        let mut rx = self.receiver.lock().await;
        let msg = rx.recv().await.ok_or(MBrokerError::CantReadMessage(
            "Message can't be received".to_string(),
        ))?;
        let payload = msg.content.ok_or(MBrokerError::CantReadMessage(
            "Something went wrong with the content of the message".to_string(),
        ))?;
        let ack_args = BasicAckArguments::new(msg.deliver.unwrap().delivery_tag(), false);
        ch.basic_ack(ack_args).await.unwrap();
        println!(" [x] Received");
        Ok(payload.into())
    }
}
