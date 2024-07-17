pub mod server{

    use std::{collections::HashMap,
            io::{BufReader, Error, Read, Write},
            net::{TcpListener, TcpStream},
            sync::{Arc, Mutex},thread};
    use crate::queue::queue::{new_queue, Queue};


    // Whenever a new client joins the Stream, His TcpStream along with Client_id will be stored in **clients** Vector
    // So whenever a clients sends a message it will be stored in **queues** Vector
    // Which will be processed by Server to get client_id and Message
    // He has to specify the client_id in the format(send:client_id message)

    pub struct Server{
        clients: Arc<Mutex<HashMap<usize,TcpStream>>>,
        queues: Arc<Mutex<Queue>>
    }


    // create a new server
    pub fn new_server() -> Server {
        let clients = Arc::new(Mutex::new(HashMap::new()));

        // Currently we only need a single queue which contains all the messages from the clients
        let queues = Arc::new(Mutex::new(new_queue())); 
        
        let server = Server{clients,queues};
        return server;
    }

    impl Server{

        // start a Server
        pub fn start(self){
            println!("Starting message-queue server...\n");

            let conn_listner = TcpListener::bind("127.0.0.1:8080").unwrap();

            let mut cid = 0;

            for stream in conn_listner.incoming(){
                let stream = stream.unwrap();

                let mut queue_ref = Arc::clone(&self.queues.clone());
                let mut clients = Arc::clone(&self.clients.clone());

                cid += 1;

                //adding a new client to clients vector
                {
                    let mut clients_ref = self.clients.lock().unwrap();
                    let stream_ref = stream.try_clone().unwrap();
                    clients_ref.insert(cid,stream_ref);

                    let mut registered_client = clients_ref.get(&cid).unwrap().try_clone().unwrap();
                    send_confirmation(&mut registered_client,format!("You have been Registered with Client_id:{cid}")).unwrap();

                }// clients_ref goes out of scope -- mutex guard is unlocked

                thread::spawn(move|| {
                    handle_connection(&mut queue_ref,&mut clients,stream,&cid);
                });
            }
        }

    }


    //client handler --> handling a single connection
    fn handle_connection(
        queues: &mut Arc<Mutex<Queue>>,
        clients:&mut Arc<Mutex<HashMap<usize,TcpStream>>>,
        mut stream: TcpStream,
        cid: &usize
    )
    {

        loop{
            //ie:: Buffreader is used when we are reading something from Stream like Network stream, Files
            // This reduces System Calls
            let mut buffer = [0;512];
            let mut reader = BufReader::new(&mut stream);
            let bytes_read = reader.read(&mut buffer).unwrap();

            if bytes_read == 0 {
                println!("Client:{} Disconnected from server",cid);
                break;
            }

            //converts the bytes of Stream from utf8 into readable String
            let request = String::from_utf8_lossy(&buffer).to_string();


            //supports only one operation: send{client_id} message

            // extract operation and message from the stream
            let (operation, message) = extract_request(&request);

            if operation.starts_with("quit"){
                println!("Client:{} Disconnected from server",cid);
                break;
            }


            if operation.starts_with("send:"){
                let client_id:usize = operation[5..].parse().unwrap();


                println!("This Print Works");

                let mut queue_ref = queues.lock().unwrap();
                queue_ref.add_message(message.to_string());

                let mut clients_ref = Arc::clone(&clients);
                // write to that particular TcpStream
                send_private_message(&client_id,message.to_string(),&mut clients_ref);
                // return;
            }

            //messages will print in Server
            if operation.starts_with("getallmessagesinqueue"){
                let mut queue_ref = queues.lock().unwrap();
                queue_ref.retrive_message();
                // return;
            }

        }

        // println!("Invalid Opearation! Try again");
    }

    fn extract_request(s:&str) -> (&str,&str){
        //splitn function splits the string into atmost two parts based on the delimmiter
        let result:Vec<&str> = s.splitn(2,' ').collect(); 

        if result.len() > 1 {
            return (&result[0],&result[1]);
        }
                  
        return (&result[0],"dummy"); 
    }

    fn send_confirmation(stream: &mut TcpStream,message: String) -> Result<(),Error>{
        stream.write_all(message.as_bytes()).unwrap();
        Ok(())
    }

    fn send_private_message(client_id: &usize,message: String,clients: &mut Arc<Mutex<HashMap<usize,TcpStream>>> ){

        let clients_ref = clients.lock().unwrap();

        let mut stream = clients_ref.get(client_id).unwrap();
        stream.write_all(message.as_bytes()).unwrap();
    }




}