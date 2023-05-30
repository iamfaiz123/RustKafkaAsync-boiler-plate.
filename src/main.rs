use actix_web::{web, App,HttpServer};
use lib::kafka::kafka_producer::create_producer;
/// since flume sender and reciever impls send and sync we can spawn receiver into another thread
use lib::{self, error::CoreError,api::send_email::send_email_api};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), CoreError> {
    // load configs
    let config = lib::config_loaders::build_config()?;
    // build kafka topic
    let _ = lib::kafka::create_topic::create_topic(&config.kafka_topic_email, &config).await?;
    // get kafka producer and wrap as web data
    let kafka_producer = create_producer(&config)?;
    let kafka_producer = web::Data::new(kafka_producer);

    let _ = HttpServer::new(move || {
        App::new().app_data(kafka_producer.clone())
        .route("/send-mail", web::post().to(send_email_api))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
