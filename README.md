# Tutorial 10 reflection

## Experiment 2.1: Original code, and how it run

**Server**
![image](https://github.com/Sirered/adprog-tutorial10-chatasync/assets/126568984/38cc1f40-ae78-4c72-82ed-46ee46959e3f)

**Client 1**
![image](https://github.com/Sirered/adprog-tutorial10-chatasync/assets/126568984/867d1c46-662c-43db-a244-c031d82c0f5e)

**Client 2**
![image](https://github.com/Sirered/adprog-tutorial10-chatasync/assets/126568984/6dbe876f-ab4e-4c5d-b849-90fc03acc7da)

**Client 3**
![image](https://github.com/Sirered/adprog-tutorial10-chatasync/assets/126568984/7cb5221b-cda2-4fc4-92da-3878cb67e7c4)

**How to run and what happens**

First you have to setup/execute the server, by running cargo run --bin server. This will run the server who will listen in on port 2000 for connections, which it will receive messages from. It will process these messages and send/broadcast it to all of the clients using those connections. Then we run multiple clients using cargo run --bin client, who will listen on some unused port and establish a connection to the server. When we establish a new connection, it will open a new thread that initiates a loop in which it uses tokio::select! to handle events. Specifically, when there is a message in the broadcast receiver, it will broadcast a message to all clients, and when there is a new message in the connection reader, it will print the message and add it to the broadcast receiver. We can also see the fact that a new connection was made on the server's terminal.  

Similar to the server's handle connection, a loop is initialised on the client when it is run that will handle events. Specifically, if it receives a message from the stream reader, it will print it on the terminal, and if a line is inputted on the terminal, that line will be made into a message and is sent to the server. So overall, this is the rundown of what happens:

* user types and enters a line on the client's terminal
* client sends that line as a message to the server
* the server receives the message, prints it on terminal and puts it in the broadcast receiver
* the server's broadcast receiver reseives that message, which it will send/broadcast to all clients connected to it
* all clients connected to the server receive the message and outputs it on their terminal

## Experiment 2.2: Modifying port

To edit the port that the server is listening in on, we just have to edit the port number in the address that is inputted in the TcpListener::bind methodfrom 2000 to 8080. If we left it at that, the server would be listening in on port 8080, but the client would still be trying to establish a connection to port 2000, thus breaking the code. To fix this we also edit the ClientBuilder::from_uri method so that the input is the adddress with the new port, thus making it so that the client will try to establish a connection to port 8080 (thus allowing the server and client work again). 

As for what websocket protocol is used and if they are the same, from what I can tell they do indeed use the same package, specifically the tokio_websockets package to establish websocket connections. Since they use the same package to do so, they must also use the same protocol. In the last 2 screenshots of this section we will go through how a websocket connection is established.

First using tokio's net method, we have the server listen in on the address and port we have set. Next we have a loop that awaits a tcp connection to occur before going through the entire loop. If a tcp connection is established with the server, the listener will accept it, returning a variable of tokio's TcpStream struct that will be used for communication. After that, the server will use tokio_websockets' ServerBuilder to build a web socket server connection that can receive and send messages to the client, utilising the TcpStream we established earlier, at which point a thread is used to handle all of the receiving and sending from and to that connection. On the client's side, it will use tokio_websockets' ClientBuilder to establish a TCP stream with the server, thus establishing a web socket client connection. The server and client websocket connections are both necessary to facilitate websocket communication between the 2 and they are both established using tokio_websockets, thus they use the same protocol.

**Change in server**
![image](https://github.com/Sirered/adprog-tutorial10-chatasync/assets/126568984/b7010d93-9dce-471b-87b6-33feba688408)

**Change in client**
![image](https://github.com/Sirered/adprog-tutorial10-chatasync/assets/126568984/772583cd-52f2-47bf-bda7-728e11a7b872)

**Imports to show where methods and classes come from**
![image](https://github.com/Sirered/adprog-tutorial10-chatasync/assets/126568984/c30bc7f4-87c3-44b1-8b5d-52ade13654ec)
![image](https://github.com/Sirered/adprog-tutorial10-chatasync/assets/126568984/83e2bb8c-fc18-4c34-b59b-2d9da86b1155)

**Server websocket protocol**
![image](https://github.com/Sirered/adprog-tutorial10-chatasync/assets/126568984/3356c8ed-a684-436b-978c-ba8623c52796)
