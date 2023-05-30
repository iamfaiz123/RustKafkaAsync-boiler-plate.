use rdkafka::{producer::FutureProducer, ClientConfig};

use crate::{config_loaders::EnvConfigs, error::CoreError};

// Function to create a Kafka producer
pub fn create_consumer(settings: &EnvConfigs) -> Result<FutureProducer, CoreError> {
    // Create a new FutureProducer with the Kafka broker URL from settings
    let producer: FutureConsumer = ClientConfig::new()
        .set("bootstrap.servers", &settings.kafka_broker_url)
        .set("message.timeout.ms", "5000")
        .create()?;

    Ok(producer)
}