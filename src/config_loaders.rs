// Import necessary dependencies
use crate::error::CoreError;
use config::{Config, FileFormat};

// Define the structure for environment configurations

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
pub struct Application {
    pub host: String,
    pub port: u16,
}
#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
pub struct EnvConfigs {
    #[serde(rename = "kafka-broker-url")]
    pub kafka_broker_url: String,
    #[serde(rename = "kafka-topic-email")]
    pub kafka_topic_email: String,
    #[serde(rename = "Application")]
    pub application: Application,
}

// Function to build the configuration
pub fn build_config() -> Result<EnvConfigs, CoreError> {
    // Create a new configuration builder
    let config = Config::builder();

    // Add a JSON file source named "config"
    use config::File;
    let config = config.add_source(File::new("config", FileFormat::Json));

    // Build the configuration and try to deserialize into EnvConfigs
    let envs = config.build()?.try_deserialize()?;

    // Return the deserialized environment configurations
    Ok(envs)
}
