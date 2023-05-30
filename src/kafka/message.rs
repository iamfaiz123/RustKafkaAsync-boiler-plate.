use rdkafka::{message::ToBytes, producer::FutureRecord};
use serde::Serialize;

#[derive(serde::Serialize)]
pub struct KafkaMessage<'a, T: Serialize> {
    payload: T,
    bytes: Vec<u8>,
    phantom_data: std::marker::PhantomData<&'a T>,
}

impl<'a, T: Serialize> KafkaMessage<'a, T> {
    // Constructor for KafkaMessage
    pub fn new(data: T) -> Self {
        // Serialize the payload into bytes using bincode
        let message_bytes = bincode::serialize(&data).unwrap();

        // Create a new KafkaMessage with the payload and serialized bytes
        Self {
            payload: data,
            bytes: message_bytes,
            phantom_data: std::marker::PhantomData,
        }
    }

    // could use 'static str as well
    pub fn get_future_record(&'a self, topic: &'a str) -> FutureRecord<(), KafkaMessage<T>> {
        FutureRecord::<(), KafkaMessage<T>>::to(topic).payload(self)
    }
}

impl<'a, T: Serialize> ToBytes for KafkaMessage<'a, T> {
    // Implementing the `to_bytes` method from the `ToBytes` trait
    fn to_bytes(&self) -> &[u8] {
        // Return a reference to the byte slice
        &self.bytes[..]
    }
}
