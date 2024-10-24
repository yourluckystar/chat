use tokio::net::TcpStream;
use crossterm::terminal;
use tokio::sync::Mutex;
use std::sync::Arc;

mod io;
mod ui;
mod util;

#[tokio::main]
async fn main() { 
    let config = config::get_config();

    let address = format!("{}:{}", config.server.server_ip, config.server.server_port);
    let stream = TcpStream::connect(address).await.expect("Failed to connect");
    let (reader, writer) = stream.into_split();

    terminal::enable_raw_mode().unwrap();
    print!("\x1B[2J\x1B[3J\x1B[H");

    ui::footer::draw();

    let shared_variable = Arc::new(Mutex::new(String::new()));

    let read_shared = Arc::clone(&shared_variable);
    let _read_task = tokio::spawn(async move {
        io::read_message::read_message(reader, read_shared).await;
    });

    let input_shared = Arc::clone(&shared_variable);
    let input_task = tokio::spawn(async move {
        io::input_handler::input_handler(writer, input_shared).await;
    });

    let _ = tokio::join!(input_task);
}
