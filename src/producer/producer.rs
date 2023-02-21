use rdkafka::{producer::{FutureProducer, FutureRecord, future_producer::OwnedDeliveryResult}, ClientConfig};

pub struct ProducerKafka {
    producer: FutureProducer,
    topic: String,
}

impl ProducerKafka {
    pub fn new(servers : &str, username: &str, password: &str, topic: &str) ->Self {
        let producer = ClientConfig::new()
        .set(
            "bootstrap.servers",
            servers,
        )
        .set("sasl.mechanism", "SCRAM-SHA-256")
        .set("security.protocol", "SASL_SSL")
        .set(
            "sasl.username",
            username,
        )
        .set("sasl.password", password)
        .create()
        .expect("Producer creation error");
    
        ProducerKafka {  
            producer,
            topic: topic.to_string()
        }
    }

    pub async fn send(&self, key: &str, payload: String) -> OwnedDeliveryResult {
        let data = FutureRecord::to(&self.topic).payload(&payload[..]).key(key);
        let result = self.producer.send(data, None).await?;
        Ok(result)
        
    }
}