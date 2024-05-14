use std::net::{TcpStream};
use std::io::Read;
use std::io::Write;
use log::{info, warn};
use protocol::{Message, ParseResult};

fn main() {
    env_logger::init();
    info!("Starting server...");
    let mut stream = TcpStream::connect("192.168.44.21:8080").expect("Failed to connect to server");
    info!("Connection established!");

    loop {
        // send a ping message
        let postcard = protocol::wrap_msg(Message::Ping { id: 1 }).expect("Unable to wrap message");
        stream.write(postcard.as_slice()).unwrap();
        info!("Request: {:?}", postcard);

        // read the response
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let parse_result: ParseResult = protocol::parse(&buffer);
        match parse_result {
            ParseResult::HeaderInvalid => {
                warn!("Header invalid");
                continue;
            }
            ParseResult::Need(n) => {
                warn!("Need more data: {}", n);
                continue;
            }
            ParseResult::DataInvalid => {
                warn!("Data invalid");
                continue;
            }
            ParseResult::Found(msg) => {
                info!("Received message: {:?}", msg);
            }
        }

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
