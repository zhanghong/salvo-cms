use dotenvy::dotenv;
use lapin::message::DeliveryResult;

mod service;

use cms_core::config::AppState;
use crate::service::RabbitMQService;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let state = AppState::init().await;

    let pool = &state.rabbitmq;
    let queue_name = "queue_test";
    let queue_tag = "";
    let channel = RabbitMQService::init_channel_and_queue(pool, queue_name).await.unwrap();
    
    let consumer = RabbitMQService::init_consumer(&channel, queue_name, queue_tag).await.unwrap();

    consumer.set_delegate(move |delivery: DeliveryResult| async move {
        let delivery = match delivery {
            // Carries the delivery alongside its channel
            Ok(Some(delivery)) => delivery,
            // The consumer got canceled
            Ok(None) => return,
            // Carries the error and is always followed by Ok(None)
            Err(error) => {
                dbg!("Failed to consume queue message {}", error);
                return;
            }
        };

        // Do something with the delivery data (The message payload)
        println!("Received message data: {:?}", delivery.data);

        RabbitMQService::delivery_basic_ack(&delivery).await.unwrap();
    });

    let payload = b"Hello world!";
    let exchange = "";
    let routing_key = "queue_test";
    RabbitMQService::publish_message(pool, exchange, routing_key, payload).await.unwrap();

    std::future::pending::<()>().await;
}