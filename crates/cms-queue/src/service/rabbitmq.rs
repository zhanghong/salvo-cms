use deadpool_lapin::Pool;
use deadpool_lapin::lapin::{
    options::BasicPublishOptions,
    BasicProperties,
};
use lapin::message::Delivery;
use lapin::Consumer;
use lapin::{options::{QueueDeclareOptions, BasicAckOptions, BasicConsumeOptions}, types::FieldTable, Channel};

use cms_core::domain::{handle_ok, HandleResult};

pub struct RabbitMQService {}

impl RabbitMQService {
   pub async fn init_channel_and_queue(pool: &Pool, queue_name: &str) -> HandleResult<Channel> {
    let connection = pool.get().await?;
        let channel = connection.create_channel().await?;
        let _queue = channel
            .queue_declare(
                queue_name,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;
        
        handle_ok(channel)
   } 

   pub async fn publish_message(pool: &Pool, exchange: &str, routing_key: &str, payload: &[u8]) -> HandleResult<()> {
    let connection = pool.get().await?;
    let channel = connection.create_channel().await?;

    channel.basic_publish(exchange, routing_key, BasicPublishOptions::default(), payload, BasicProperties::default()).await
    .unwrap()
    .await
    .unwrap();

    handle_ok(())    
   }

   pub async fn init_consumer(channel: &Channel, queue_name: &str, queue_tag: &str) -> HandleResult<Consumer> {
    let consumer = channel
    .basic_consume(
        queue_name,
        queue_tag,
        BasicConsumeOptions::default(),
        FieldTable::default(),
    )
    .await?;

    handle_ok(consumer)
   }

   pub async fn delivery_basic_ack(delivery: &Delivery) -> HandleResult<()> {
    delivery
    .ack(BasicAckOptions::default())
    .await
    .expect("Failed to ack send_webhook_event message");

    handle_ok(())
   } 
}