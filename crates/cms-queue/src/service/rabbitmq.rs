use deadpool_lapin::Pool;
use deadpool_lapin::lapin::{BasicProperties, options::BasicPublishOptions};
use lapin::Consumer;
use lapin::message::Delivery;
use lapin::{
    Channel,
    options::{BasicAckOptions, BasicConsumeOptions, QueueDeclareOptions},
    types::FieldTable,
};

use cms_core::domain::{HandleResult, handle_ok};
use cms_core::error::AppError;
use tracing::{error, info}; // 确保引入 AppError

pub struct RabbitMQService {}

impl RabbitMQService {
    pub async fn init_channel_and_queue(pool: &Pool, queue_name: &str) -> HandleResult<Channel> {
        if queue_name.is_empty() {
            return Err(AppError::BadRequest("Queue name cannot be empty".to_string()).into());
        }

        let connection = pool.get().await?;
        let channel = connection.create_channel().await?;

        // Declare the queue with proper error handling
        let _queue = channel
            .queue_declare(
                queue_name,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        info!("Queue '{}' initialized successfully", queue_name);
        handle_ok(channel)
    }

    pub async fn publish_message(
        pool: &Pool,
        exchange: &str,
        routing_key: &str,
        payload: &[u8],
    ) -> HandleResult<()> {
        if (exchange.is_empty() && routing_key.is_empty()) || payload.is_empty() {
            let error = AppError::BadRequest(
                "Exchange, routing key, and payload must not be empty".to_string(),
            );
            return Err(error.into());
        }

        let connection = pool.get().await?;
        let channel = connection.create_channel().await?;

        match channel
            .basic_publish(
                exchange,
                routing_key,
                BasicPublishOptions::default(),
                payload,
                BasicProperties::default(),
            )
            .await
        {
            Ok(publish_result) => {
                if let Err(err) = publish_result.await {
                    error!("Failed to confirm message publication: {}", err);
                    return Err(err.into());
                }
                info!(
                    "Message published successfully to exchange '{}' with routing key '{}'",
                    exchange, routing_key
                );
                handle_ok(())
            }
            Err(err) => {
                error!("Failed to publish message: {}", err);
                Err(err.into())
            }
        }
    }

    pub async fn init_consumer(
        channel: &Channel,
        queue_name: &str,
        queue_tag: &str,
    ) -> HandleResult<Consumer> {
        if queue_name.is_empty() && queue_tag.is_empty() {
            return Err(AppError::BadRequest(
                "Queue name and queue tag can not both empty".to_string(),
            )
            .into());
        }

        let consumer = channel
            .basic_consume(
                queue_name,
                queue_tag,
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        info!("Consumer initialized for queue '{}'", queue_name);
        handle_ok(consumer)
    }

    pub async fn delivery_basic_ack(delivery: &Delivery) -> HandleResult<()> {
        if let Err(err) = delivery.ack(BasicAckOptions::default()).await {
            error!("Failed to acknowledge message: {}", err);
            return Err(err.into());
        }

        info!("Message acknowledged successfully");
        handle_ok(())
    }
}
