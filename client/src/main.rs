use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::io::Write;
use log::info;

fn main() {
    env_logger::init();
    info!("Starting server...");
    let mut stream = TcpStream::connect("192.168.44.21:8080").expect("Failed to connect to server");
    info!("Connection established!");

    loop {
        stream.write(b"Ping!").unwrap();
        info!("Request: Ping!");
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        info!("Response: {}", String::from_utf8_lossy(&buffer));

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    // for stream in listener.incoming() {
    //     env_logger::init();
    //
    //     let mut stream = stream.unwrap();
    //     info!("Connection established!");
    //
    //     loop {
    //         let mut buffer = [0; 1024];
    //         stream.read(&mut buffer).unwrap();
    //         info!("Request: {}", String::from_utf8_lossy(&buffer));
    //
    //         stream.write(b"Pong!").unwrap();
    //     }
    // }
}
