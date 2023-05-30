use actix_web::web;
use rdkafka::producer::FutureProducer;
use actix_web::Responder;
use crate::error::ApiError;
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Email {
    email: String,
    body: String,
}

pub async fn send_email_api(kafka_producer: web::Data<FutureProducer>, from: web::Json<Email>)->Result<impl Responder,ApiError> {
    //create a new kafka msg
    use crate::kafka::message::KafkaMessage;
    let kafka_message = KafkaMessage::new(from.into_inner());
    let record = kafka_message.get_future_record("email");
    let _ = match kafka_producer.send(record, None).await{
        Ok(_)=>{}
        Err((kafka_err,_))=>{
              return Err(ApiError::from(kafka_err))
        }
    };

    Ok("message send")
}
