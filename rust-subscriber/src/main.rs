// Port of https://www.rabbitmq.com/tutorials/tutorial-one-python.html. Run this
// in one shell, and run the hello_world_publish example in another.
use amiquip::{Connection, ConsumerMessage, ConsumerOptions, QueueDeclareOptions, Result};


fn split_message(input_string: &str) -> Vec<String> {
    let vec = input_string.split_whitespace().map(str::to_string).collect();
    vec
}


fn main() -> Result<()> {
    // Open connection.
    let mut connection = Connection::insecure_open("amqp://guest:guest@rabbitmq:5672")?;

    // Open a channel - None says let the library choose the channel ID.
    let channel = connection.open_channel(None)?;

    // Declare the "hello" queue.
    let queue = channel.queue_declare("python-testing", QueueDeclareOptions::default())?;

    // Start a consumer.
    let consumer = queue.consume(ConsumerOptions::default())?;
    println!("Waiting for messages. Press Ctrl-C to exit.");

    for (_i, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body);
                let vector_of_strings = split_message(&body.to_string());
                println!("RUST_MESSAGE_RECEIVED -> {:?}", vector_of_strings);
                consumer.ack(delivery)?;
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }

    connection.close()
}
