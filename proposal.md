# Real Time Chat Application
# ECE1724 Project Proposal

#### Chujing Yu 1010809608, Minghao Ma 1011818428, Yiyang Liu 1011770512

## Motivation

Real-time communication tools have become indispensable in recent years, transforming how people connect and collaborate. Popular applications like WeChat, Discord, and Microsoft Teams offer features such as instant messaging, file sharing, and collaborative spaces. 

However, these widely used tools are often built using languages like JavaScript, C++, or Java. While effective, these languages present limitations, particularly in performance and memory management, as they often struggle with high concurrency demands and real-time responsiveness under heavy loads.

Rust, on the other hand, is a systems programming language known for its memory safety, zero-cost abstractions, and high concurrency support, making it particularly well-suited for building high-performance, reliable real-time communication applications. Rust’s strict compile-time checks, efficient resource handling, and robust error handling provide significant advantages, reducing the risk of runtime errors and ensuring smoother performance even when the system is under pressure.

This project is motivated by our desire to leverage Rust’s strengths to create a fast, scalable, and reliable real-time chat application. By doing so, we aim to contribute to the Rust ecosystem, which currently lacks open-source real-time communication solutions, and to explore the untapped potential of Rust in this field.

## Objective and key features

In this project, we aim to develop a real-time chat application using Rust that is capable of handling multiple users and high concurrency while maintaining stability and security. This application will include several core functions:

* User Authentication and Presence Detection: Users will be able to register, log in, and log out with their unique accounts. Their online status will be updated in real time, ensuring that other users in shared chat rooms are aware of who is active or inactive.
* Chat Room Creation and Management: Users can create or join chat rooms with unique IDs, and they can participate in multiple chat rooms simultaneously. The system will handle HTTP requests from users to manage chat room operations and efficiently track each room’s active users.
* Real-Time Messaging with WebSockets: Users in the same chat room can send and receive text messages in real time, enabled by a WebSocket server for message routing and broadcast. This server will also handle connection management, message filtering, and other essential tasks.
* Scalability and Security: The application will leverage Rust’s concurrency and memory safety features to ensure that it remains robust and efficient even under high concurrency. Additionally, it will be designed to resist common security threats, enhancing the reliability of real-time communications.

The distribution of tasks among team members is detailed below to ensure a balanced workload that maximizes each member’s strengths.



## Tentative plan

### 1. Real-time messaging using WebSockets

This task involves setting up the WebSocket server for real-time messaging.

1. **WebSocket Connection Establishment**

    The server and user files will be run to set up an initial WebSocket connection.This initial connection will enable the server to handle basic commands, like login and registration, over an established connection. User Authentication via Commands: Once the connection is established, users will send commands to either log in or register, using the connected WebSocket. This approach allows for flexibility, as commands like /login or /register can be handled by the server to authenticate users.

2. **Message Handling**
   
    * **Receiving Messages**
  
      After logging in, users can send messages to the server. To help the server decide how to handle each message, we will define a "Message" struct containing details such as message type, plain text, user ID, and chat room ID.

     * **Processing Messages**
      
       The server will classify messages based on type (e.g., command or text message). Commands, starting with /, will be matched to functions like joining or creating rooms, while text messages will be routed to other users in the chat room.

    * **Sending Messages**
      
      When the server receives a text message from a user, it broadcasts the message to other users in the same room. Additionally, if a new user joins, the server will send an automatic notification to room members.

3. **Scalability Considerations**

    * **concurrent situation** 

       The asynchronous capabilities of Rust-WebSocket are expected  to enable each WebSocket connection to operate as an independent asynchronous task, reducing competition and blocking among threads. Actix’s HttpServer automatically starts multiple worker threads, allowing concurrent message handling. This structure will ensure the application remains efficient even with multiple users interacting in real time.

### 2. Chat room creation and joining

This module provides users with the ability to create new chat rooms and join existing ones. Each room will have a unique ID for users to join and communicate with others within the same room.

1. **Feature Design**

    To create a chat room, users can initiate a request, and the system will assign a unique ID to each new room, allowing others to join by entering this ID. When a chat room is created, the system can initialize a user list. For joining, users can enter the chat room ID to join an existing room. Upon successfully joining, the system will update the user list by adding the new user and establishing a WebSocket connection to facilitate message exchange. If the user joins an already active chat room, the system can load the list of existing users' IDs.


2. **Technical Implementation**

   To manage multiple chat rooms, a HashMap is expected to store information, with each room’s ID as the key and a struct containing room details (ID and user list) as the value. This setup will allow dynamic updating of user lists, removing users from the list when they leave the room. When a user joins a chat room, a WebSocket connection is established for message exchange, with the WebSocket server routing messages based on the chat room ID, ensuring they are sent only within the designated room. Using WebSocket broadcasting, the system will send messages to all connected users in the same chat room. Each chat room should be handled as an asynchronous task, and Rust’s tokio library is expected to be utilized to implement asynchronous WebSocket handling, ensuring independent message processing across chat rooms.

3. **Error Handling**

    Error handling should include chat room ID validation, where the system returns an error if a user attempts to join a non-existent chat room. If a chat room is full or restricted, the system will also provide appropriate feedback. When a user leaves a chat room, they are automatically removed from the user list.


### 3. User Authentication and Presence Detection

This module focuses on implementing authentication mechanisms and presence detection functionality.

1. **Authentication System**

    To ensure secure access to chat rooms and provide users with personalized sessions, a lightweight authentication system is expected to implement to allow registration, login, logout, and session management.

    * **User Registration**

       New users will register by creating a unique username and password, which will be stored in a secure format within the system(e.g. hash map).  During registration, the system will check the uniqueness of each username, preventing duplicates to ensure individual identity for each user within the application.

    * **User Login and Session Management**

       Once registered, users can log in using their chosen username and password.  Upon successful authentication, the system generates a session token that is stored server-side and associated with the user's session. This token-based approach allows users to stay authenticated as they interact with the chat app, which provides continuity throughout their session.

    * **Logout Functionality**

       Users will have a logout option, which will invalidate their session token and end their authenticated session. This process will clear any authentication data tied to the user, ensuring that any subsequent access attempts require re-authentication, securing the application from unauthorized access.

2. **Presence Detection**

   Presence detection will be implemented by monitoring users' login and logout events. When a user logs in, they will be marked as "online" in the system; when they log out, their status will update to "offline".To ensure reliability, the system will perform periodic session checks to mark users who may have lost connection as offline after a timeout. When marked as offline, the system will automatically invalidate the session token to prevent unauthorized access.

### 4. Frontend Development

A simple command-line interface (CLI) will allow users to interact with the chat application. Through the CLI, users can authenticate, create/join rooms, send/receive messages, and view other users’ statuses. The CLI will connect with the backend via WebSocket connections and RESTful API calls for user actions.

### 5. Team Responsibility

|**Feature**   |**Team Member**|
|--------------|---------------|
|Real-Time Messaging|Minghao Ma|
|Chat Room Creation & Joining|Yiyang Liu|
|User Authentication & Presence Detection|Chujing Yu|
|Frontend Development                    |All Members|
|Testing and Debugging                   |All Members|
|Final Review(source code, description and video demo)|All Members|
