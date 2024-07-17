## ClientsChat

- Simple Multithreaded Client Server architecture from Scratch  
- Built using Standard Libraries
- Used Shared Memory to communicate between threads


Run the server 
```
cd server && cargo run 
```

Run the Client
```
rustc client.rs && ./client
```


## Features 

- Communicate b/w different Clients 


## Commands -- clients side 

send private message to another client
`
send:{client_id} message
`

getallmessages  -- messages will be displayed in the server
`
getallmessages
`

quit
`
quit
`





## Different ways to establish Multithreaded Communication:

1. **Shared Memory**:
    - To prevent the case where multiple Threads trying to access the same resource  
    at same time  
    - We can add a Mutex( MutuallyExclusive) Guard
	- By using Mutex Guard 
		- By default the resource is Locked(or no one can acesss it)
		- If any Threads wants to acesss it it Then it has to unlock it 
		- Then it will be exclusively available to that Thread
		
	- In case of Rust: 
		- As soon as the variable goes out of the scope, the resource is unlocked and any other thread can access it
		- There might be a case of DeadLock, where a Thread is always trying to access the resource while it's locked 
		- So it's a necessary part in remove the lock

2. **Message passing**:

    - Use message passing to create a channel b/w two Threads 
    - Using Channel we can send and Recieve data b/w threads 
	- In case of Rust: We use MPSC(Multiple Producer Single consumer)
		- Where you can get data from different Threads into main Thread








