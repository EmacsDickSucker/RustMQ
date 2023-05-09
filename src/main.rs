// Port of https://www.rabbitmq.com/tutorials/tutorial-one-python.html. Start the
// hello_world_consume example in one shell, and run this in another.
use amiquip::{Connection, Exchange, Publish, Result};
use std::env;

fn main() -> Result<()> {
    env_logger::init();

    send_message()
}

fn send_message() -> Result<()> {
    // Open connection.

    let server_url = env::var("URL").unwrap_or("amqp://guest:guest@localhost:5672".to_string());
    let mut connection = Connection::insecure_open(&server_url)?;

    // Open a channel - None says let the library choose the channel ID.
    let channel = connection.open_channel(None)?;

    // Get a handle to the direct exchange on our channel.
    let exchange = Exchange::direct(&channel);

    // Publish a message to the "hello" queue.
    exchange.publish(Publish::new(b"hello there", "hello"))?;

    connection.close()
}
