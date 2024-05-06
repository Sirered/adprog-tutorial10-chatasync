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
