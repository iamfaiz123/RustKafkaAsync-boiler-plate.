This is the producer microservice that will send message to kafka ```email``` topic,
since sending email is an important thing and it can take time if we smpt , we can decouple our api using kafka.

# start zoo keeper from installed kafka directory
``` bin/zookeeper-server-start.sh config/zookeeper.properties ```
# start kafka server
``` bin/kafka-server-start.sh config/server.properties ```
## code will create topics in server for you.
# run code
```cargo run ```
