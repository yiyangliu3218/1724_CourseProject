# Real Time Chat Application

#### Chujing Yu, Minghao Ma, Yiyang Liu

## Motivation

Real-time communication tools have become indispensable in recent years, reshaping how people connect and collaborate. Apps like WeChat, Discord, and Microsoft Teams have provided users with instant messaging, file sharing, and collaborative spaces.  

However, many of these popular tools are built using languages like JavaScript, C++, or Java, which, while effective, have limitations in terms of performance and memory management. Rust, in constrast, offers several advantages that could enhance the performance and reliability of real-time communication applications. Know for its memory safety, zero-cost abstractions, and high concurrency support, Rust provides a powerful foundation for building systems where performance is crucial. Its strict compile-time checks and efficient handling of resources help reduce the risk of runtime errors and ensure smoother performance even under heavy loads.

This project is motivated by our desire to leverage Rust's strengths to create a fast, scalable, and reliable real-time chat application. We hope to contribute to the relatively untapped potential of Rust in the field of real-time communication, where few open-source projects are readily available.

## Objective and key features

In this project, we aim to complete a Rust based real-time chat application. It is considered to have the following functions and features:

* Every user can register for an account and log in to the application with his/her account. When they successfully log in or log out, their online state will be updated. This requires a user authentication and presence detection system.
* Users can create or join a chat room. A user can enroll in multiple chat rooms at the same time. This requires to set an HTTP server, which can receive requests from users, and manage the chat room according to the requests.
* People can send text message in the chat room, which can be seen by each other. This requires a WebSocket server which is responsible for the connection management, real-time message transmission, message filtering and other basic tasks.
* Besides, we hope that the chat application should be robust and safe, which means it can handle high concurrency scenarios and defend against some basic attacks. This is a meaningful practice to make use of Rust's advantages, such as memory safety, and high concurrency support. 

The team responsibility is showed in the end of the proposal, which clarified the responsibilities of each member's tasks.


## Tentative plan

### 1. Real-time messaging using WebSockets

This task will focus on supporting Real-time messaging using WebSockets

1. **WebSocket Integration**

    To make Real-time communication possible, we will exploit Rust-Websocket to implement the websocket server, and use Actix Web to implement the HTTP server. Once the client file is executed, the client tries to establish a connection with the server through websocket protocol. If the connection is established, the client and the server can communicate with each other.
   
    * **Message reseiving**
  
      The server should be able to receive messages from a user. To help the server decide what to do with the message, we need to define a "Message" struct. It should contain the plain text, the client's id, and which chat room it comes from. We can define the struct by ourselves or using the existing struct provided by Rust-Websocket. The HttpServer type in Actix-Web can be exploited to serve HTTP requests. The keep-alive function will be used, which lets Actix Web keep connections open to wait for subsequent requests.

     * **Message processing**
      
       Once the server receives the message, it should judge that which type of message it belongs to. Is it a command to join/create a chat room, or just a message text? We will regulate that all command messages should begin with ‘/’. If the server receives a command message, a match sentence will be used to find out which type it belongs to, and execute the command.

    * **Message sending**
      
      After the server receives text messages from a user, it should send them to other users in the same room. In other cases, such as one new user join the room, the server will also send messages off its own bat to inform other users in the same room. To implement these tasks, the websocket server should route according to the chat room ID. The Addr object of Actix-Web will be used, which provides different ways of sending the message.

3. **Scalability Considerations**

    * **concurrent situation**

      The Rust-Websocket crate has both async and sync implementations of websockets. The asynchronous features are useful in real-time messaging, since each connection can be an independent asynchronous task, which can avoid thread competition and blocking. The HttpServer in Actix automatically starts a number of HTTP workers, each worker thread processes its requests sequentially, and the number of worker threads can be overridden. With the help of these features, the chat application can handle the concurrent situation, and multiple users sending messages at the same time is acceptable.

### 2. Chat room creation and joining

This part is to provide users with the ability to create new chat rooms and join existing ones. Each chat room will have a unique ID that users can use to join specific rooms and communicate with other users within them.

1. **Feature Design**

    To create a chat room, users can initiate a request, and the system will assign a unique ID to each new room, allowing others to join by entering this ID. When a chat room is created, the system can initialize a user list and message history for the room, with a potential limit on stored messages. For joining, users can enter the chat room ID to join an existing room. Upon successfully joining, the system will update the user list by adding the new user and establishing a WebSocket connection to facilitate message exchange. If the user joins an already active chat room, the system can load previous messages to provide context.

2. **Technical Implementation**

    To manage multiple chat rooms, a `HashMap` is expected to store information where each chat room ID serves as the key, and the value is a struct containing chat room details, including the chat room ID, user list, and message history. User lists and message histories can be dynamically updated, removing users from the list when they leave the room. When a user joins a chat room, a WebSocket connection is established for message exchange, with the WebSocket server routing messages based on the chat room ID, ensuring they are sent only within the designated room. Using WebSocket broadcasting, the system will send messages to all connected users in the same chat room. Each chat room should be handled as an asynchronous task, and Rust’s `tokio` library is expected to utilized to implement asynchronous WebSocket handling, ensuring independent message processing across chat rooms.

3. **Error Handling**

    Error handling should include chat room ID validation, where the system returns an error if a user attempts to join a non-existent chat room. If a chat room is full or restricted, the system will also provide appropriate feedback. When a user leaves a chat room, they are automatically removed from the user list. 

4. **Scalability Considerations**

    To manage resources effectively, room capacity limits can be set to optimize performance and memory management, preventing resource overuse. Each chat room will also have a storage limit for messages, with old messages periodically cleaned up to free memory.

### 3. User Authentication and Presence Detection

This task will focus on implementing authentication mechanisms and presence detection functionality.

1. **Authentication System**

    To ensure secure access to chat rooms and provide users with personalized sessions, we will implement a lightweighted authentication system. This system will     allow users to register, log in, log out, and maintain authenticated sessions throughout their interaction with the chat application.

    * **User Registration**

        New users will register by creating a unique username and password, which will be stored in a secure format within the system(e.g. hash map). The                   registration     process will validate the uniqueness of usernames, preventing duplicates and ensuring each user has an individual identity within the              application.

    * **User Login and Session Management**

        Once registered, users can log in using their chosen username and password. After successful authentication, the system will generate a session token that will be stored server-side and associated with the user's session. This token-based approach will allow users to maintain authenticated as they interact with the chat app.

    * **Logout Functionality**

        Users will have a logout option, which will invalidate their session token and end their authenticated session. This process will clear any authentication data tied to the user. Once the session token is removed, any following access attempts will require re-authentication, safeguarding the app from unauthorized access.

2. **Presence Detection**

    Presence detection will be implemented by monitoring users' login and logout events. When a user logs in, they will be marked as "online" in the system; when they log out, their status will update to "offline". For additional reliability, we will do periodic session checks to ensure users who may have lost connection are correctly marked offline after a timeout. The users' "offline" status will invalidate their session token and end their authenticated session.

### 4. Frontend Development

We will develop a simple command-line client for users to interact with the chat application. The command-line client will allow users to authenticate, create/join rooms, send/receive messages, and view other users' statuses. The command-line interface will interact with the backend via WebSocket connections and RESTful API calls for users actions.

### 5. Team Responsibility

|**Feature**   |**Team Member**|
|--------------|---------------|
|Real-Time Messaging|Minghao Ma|
|Chat Room Creation & Joining|Yiyang Liu|
|User Authentication & Presence Detection|Chujing Yu|
|Frontend Development                    |          |
|Testing and Debugging                   |All Members|
|Final Review(source code, description and video demo)|All Members|
