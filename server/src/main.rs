use std::net::{ TcpListener, TcpStream };
use std::io::Write;

fn main() {

    println!("Socket created.");
    println!("Socket binded.");

    let ip = "localhost";
    let port = "1500";

    let listener = TcpListener::bind(format!("{}:{}", ip, port)).unwrap();
    
    for stream in listener.incoming() {
         match stream {
            Ok(stream) => {
                println!("Connection accepted.");
                handle_client(stream); 
            }
            Err(e) => { /* connection failed */ }
        }
    }

    println!("Socket closed.");
}

fn handle_client(stream : TcpStream) {
    println!("Connection accepted.");
    let mut writer = stream;
    writer.write(b"Hello World\r\n");

    // writer.write("Connection accepted.");
    // let mut ret = "";
    // reader.read(&mut ret);
    //stream.write(format!("Received: {}", ret));
}