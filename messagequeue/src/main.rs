use server::server::new_server;

mod server;
mod queue;

fn main() {
    
    let webserver = new_server();
    webserver.start();


}
