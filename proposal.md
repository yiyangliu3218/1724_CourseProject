# Real Time Chat Application

#### Chujing Yu, Minghao Ma, Yiyang Liu

## Motivation

Real-time communication tools have become indispensable in recent years, reshaping how people connect and collaborate. Apps like WeChat, Discord, and Microsoft Teams have provided users with instant messaging, file sharing, and collaborative spaces.  

However, many of these popular tools are built using languages like JavaScript, C++, or Java, which, while effective, have limitations in terms of performance and memory management. Rust, in constrast, offers several advantages that could enhance the performance and reliability of real-time communication applications. Know for its memory safety, zero-cost abstractions, and high concurrency support, Rust provides a powerful foundation for building systems where performance is crucial. Its strict compile-time checks and efficient handling of resources help reduce the risk of runtime errors and ensure smoother performance even under heavy loads.

This project is motivated by our desire to leverage Rust's strengths to create a fast, scalable, and reliable real-time chat application. We hope to contribute to the relatively untapped potential of Rust in the field of real-time communication, where few open-source projects are readily available.

## Objective and key features

## Tentative plan

### 1. Real-time messaging using WebSockets

This task will focus on supporting Real-time messaging using WebSockets

1. **WebSocket Integration**

    To make Real-time communication possible, we will exploit Rust-Websocket to implement the websocket server, and use Actix Web to implement the HTTP server. After a user joins a created room, it should have established a connection with the server through websocket protocol. With the help of Rust-Websocket, we hope that the functions below can be implemented to support the real-time messaging.

    * **Message reseiving and sending**
  
      The server should be able to receive messages from a user, and send them to other users in the same room. In other cases, such as one new user join the room, the server will also send messages off its own bat to inform other users in the same room. To implement these tasks, the websocket server should route according to the chat room ID, and maintains a user list for each chat room. When a user join or leave a chat room, the user list should be updated.

     * **Dealing with connections**
      
        The Rust-Websocket crate has both async and sync implementations of websockets. The synchronous features are useful in real-time messaging, since each connection can be an independent asynchronous task, which can avoid thread competition and blocking. Besides, Rust's asynchronous and concurrent features make it easy to handle a large number of concurrent WebSocket connections. Libraries like tokio and async-std can be used to implement these features.

2. **Scalability Considerations**

    * **Message storage**
  
       The messages sent by users will be stored in certain data structure in the server. They will be divided by which room they are from, and maintain the order in which they were sent. If a user enters or re-enters an existing chat room, the previous messages in the room will be retrieved. However, since we are not going to set an SQL database for now, the total size of stored messages should be limited, and every chat room will be allocated a limited storage space for the previous messages. After a chat room is deleted, the messages in this room will be deleted too.

     * **Error handle**
  
       Errors are inevitable in a real-time chat application. For example, a user may fail to receive message in a chat room because of the poor network connection. In this case, the websocket server will try to resend message to this user, and check the presence state if the resending is still failed. Other errors like an incorrect message format should also be processed properly.

### 2. Chat room creation and joining

This part is to provide users with the ability to create new chat rooms and join existing ones. Each chat room will have a unique ID that users can use to join specific rooms and communicate with other users within them.

1. **Feature Design**

    To create a chat room, users can initiate a request, and the system will assign a unique ID to each new room, allowing others to join by entering this ID. When a chat room is created, the system can initialize a user list and message history for the room, with a potential limit on stored messages. For joining, users can enter the chat room ID to join an existing room. Upon successfully joining, the system will update the user list by adding the new user and establishing a WebSocket connection to facilitate message exchange. If the user joins an already active chat room, the system can load previous messages to provide context.

2. **Technical Implementation**

    To manage multiple chat rooms, a `HashMap` is expected to store information where each chat room ID serves as the key, and the value is a struct containing chat room details, including the chat room ID, user list, and message history. User lists and message histories can be dynamically updated, removing users from the list when they leave the room. When a user joins a chat room, a WebSocket connection is established for message exchange, with the WebSocket server routing messages based on the chat room ID, ensuring they are sent only within the designated room. Using WebSocket broadcasting, the system will send messages to all connected users in the same chat room. Each chat room should be handled as an asynchronous task, and Rust’s `tokio` library is expected to utilized to implement asynchronous WebSocket handling, ensuring independent message processing across chat rooms.

3. **Error Handling**

    Error handling should include chat room ID validation, where the system returns an error if a user attempts to join a non-existent chat room. If a chat room is full or restricted, the system will also provide appropriate feedback. When a user leaves a chat room, they are automatically removed from the user list. Also,  the system should detect users who have been unresponsive for a long time and mark them as offline.

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

    Presence detection will be implemented by monitoring users' login and logout events. When a user logs in, they will be marked as "onnine" in the system; when they log out, their status will update to "offline". For additional reliability, we will do periodic session checks to ensure users who may have lost connection are correctly marked offline after a timeout. The users' "offline" status will invalidate their session token and end their authenticated session.

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
