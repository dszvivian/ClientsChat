pub mod queue{
    use std::{collections::VecDeque, sync::{Mutex, Arc}};

    // todo:  create a new enum of type -- sending, working_on, recieved


    pub struct Queue {
        // tuple of client_id, message, operation
        queue: VecDeque<String>
    }

    //create a new Queue
    pub fn new_queue() -> Queue {
        let queue = VecDeque::new();
        Queue{queue}
    }

    impl Queue{
        //add a message to queue
        pub fn add_message(&mut self,message:String){
            self.queue.push_front(message);
        }


        //pop a message from queue
        pub fn retrive_message(&mut self) {

            // if self.queue.is_empty(){
            //     return String::new();
            // }

            // return self.queue.pop_back().unwrap();

            for elem in &self.queue{
                print!("{} ",elem);
            }
        }

    }
}