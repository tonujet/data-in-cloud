use crate::config::config;
use amqprs::connection::OpenConnectionArguments;
use message_broker::error::MBrokerResult;

pub async fn get_rabbitmq_connection() -> MBrokerResult<amqprs::connection::Connection> {
    let conn = amqprs::connection::Connection::open(&OpenConnectionArguments::new(
        &config().RABBITMQ.HOST,
        config().RABBITMQ.PORT,
        &config().RABBITMQ.USER,
        &config().RABBITMQ.PASSWORD,
    ))
    .await?;
    conn.register_callback(amqprs::callbacks::DefaultConnectionCallback)
        .await
        .unwrap();
    println!("Connected to RabbitMQ");
    Ok(conn)
}
