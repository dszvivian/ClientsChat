use std::net::TcpStream;
use std::io::{self, BufRead, BufReader, Write};
use std::thread;
use std::io::prelude::*;

fn read_messages(mut stream: TcpStream) {


    let mut buffer = [0;512];


    loop{
        let bytes_read =  stream.read(&mut buffer).unwrap();

        if bytes_read == 0 {
            println!("Server has Disconnected");
            break;
        }
        
        println!("{}",String::from_utf8_lossy(&buffer));

    }
}


fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Could not connect to server");
    println!("Connected to server");
    let cloned_stream = stream.try_clone().expect("Failed to clone stream");


    //broadcasting
    thread::spawn(move || {
        read_messages(cloned_stream);
    });    

    // sending message
    loop {

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read from stdin");
        stream.write_all(input.as_bytes()).expect("Failed to write to server");

    }

}