use config::ConfigError;
use derive_more::Display;
use rdkafka::error::KafkaError;
use thiserror::Error;
use actix_web::error;
use actix_web::HttpResponse;
use actix_web::http::StatusCode;
use actix_web::http::header::ContentType;
#[derive(Debug, Error, Display)]
pub enum CoreError {
    #[display(fmt = "{}", "0")]
    EnvErrors(#[from] ConfigError),
    #[display(fmt = "{}", "0")]
    KafkaError(#[from] KafkaError),
    #[display(fmt = "{}", "0")]
    IOError(#[from] std::io::Error),
}

#[derive(Debug, Error, Display)]
pub enum ApiError {
    #[display(fmt = "things get messed up")]
    KafkaError(#[from] KafkaError),
    ValidationError,
}

impl error::ResponseError for ApiError{
    fn error_response(&self) -> HttpResponse{
        match self{
            ApiError::KafkaError(_) =>{
                HttpResponse::build(self.status_code())
                .insert_header(ContentType::html())
                .body(self.to_string())
            }
            ApiError::ValidationError=>{
                // impl this
                  unimplemented!()
            }
        }
    }
    fn status_code(&self) -> StatusCode{
        match self{
            ApiError::KafkaError(_) =>{
               StatusCode::INTERNAL_SERVER_ERROR
            }
            ApiError::ValidationError=>{
                // impl this
                  unimplemented!()
            }
        }
    }
}
