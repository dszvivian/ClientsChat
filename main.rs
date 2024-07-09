use std::net::{TcpListener,TcpStream};
use std::sync::{Arc,Mutex};
use std::thread;
use std::io::prelude::*;


fn handle_client(mut stream: TcpStream,cid:usize,clients: Arc<Mutex<Vec<(usize, TcpStream)>>>){
    let peer_addr = stream.peer_addr().unwrap();
    println!("Incoming connection from {}",peer_addr);

    {
        let mut clients =  clients.lock().unwrap();

        clients.push((cid,stream.try_clone().unwrap()));
    }

    let mut buf = [0;512];

    loop {
        let bytes_read = stream.read(&mut buf).unwrap();

        if bytes_read==0{
            println!("client {} disconnected",cid);
            break;
        }


        let recieved = String::from_utf8_lossy(&buf[..bytes_read]);

        if recieved.starts_with("send:"){
            let reciever_id: &usize = &recieved[5..6].parse().unwrap();
            let message = &recieved[7..].as_bytes();

            // println!("Recieverid: {}, message:{:?}",reciever_id,message);

            let mut clients = clients.lock().unwrap();

            private_message(*reciever_id,message,&mut clients)
        }else{
            println!("Recieved from client{}: {}",cid,recieved);


            let message = format!("Client {}: {}",cid,recieved);
            let message_bytes = message.as_bytes();
    
            let mut clients = clients.lock().unwrap();
    
            broadcast_message(&mut clients,cid,message_bytes);
        }
    }

    
}

fn private_message(sender_id:  usize, message: &[u8],clients: &mut Vec<(usize, TcpStream)>){

    for (cid,client) in clients.iter_mut(){
        if *cid == sender_id {

            println!("cid={} === recieving_id{}",*cid , sender_id);
            client.write_all(message).unwrap();
        }
    }
}


fn broadcast_message(clients: &mut Vec<(usize, TcpStream)>,sender_id:usize,message: &[u8]){

    for (cid,client) in clients.iter_mut(){
        if *cid != sender_id {
            client.write_all(message).unwrap();
        }
    }

}


fn main(){

    let clients = Arc::new(Mutex::new(Vec::new()));

    let listner = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Started Chat server at 8080");

    let mut client_id = 0;

    for stream in listner.incoming() {
        
        let stream = stream.unwrap();

        let clients_ref = clients.clone();
        client_id += 1;

        thread::spawn(move || {
            handle_client(stream,client_id,clients_ref);
        });
    }

}
