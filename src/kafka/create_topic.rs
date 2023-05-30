use crate::{config_loaders::EnvConfigs, error::CoreError};
use rdkafka::admin::AdminOptions;
use rdkafka::admin::{AdminClient, NewTopic, TopicReplication};
use rdkafka::client::DefaultClientContext;
use rdkafka::ClientConfig;

// Function to create a Kafka topic
pub async fn create_topic(topic: &str, settings: &EnvConfigs) -> Result<(), CoreError> {
    // Create a new admin client with the Kafka broker URL from settings
    let kafka_client_config: AdminClient<DefaultClientContext> = ClientConfig::new()
        .set("bootstrap.servers", &settings.kafka_broker_url)
        .set("message.timeout.ms", "5000")
        .create()?;

    // Create a new topic with one partition and replication factor of 1
    let new_topic = NewTopic::new(topic, 1, TopicReplication::Fixed(1));

    // Use the admin client to create the topic
    let _ = kafka_client_config
        .create_topics([&new_topic], &AdminOptions::new())
        .await?;

    Ok(())
}
