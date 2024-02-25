# Simple ready to use producer
This Kafka boilerplate, tailored for Rust, simplifies the integration of Kafka messaging into Rust applications. Leveraging the power of rdkafka, it streamlines data transfer with JSON data struct. Built with an async design, it seamlessly integrates with Actixweb and other actor models or async web frameworks. Whether you're orchestrating complex data pipelines or building resilient microservices, this boilerplate empowers developers with efficient Kafka integration in the Rust ecosystem.


# Simple message format 
Messages can effortlessly be sent by creating a struct with serde serialization trait implementation, along with specifying the topic name.

# dotenv configuration
All topics can be loaded from environment variables or can be hard-coded in the code itself.

