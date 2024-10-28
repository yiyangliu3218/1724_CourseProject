# Real Time Chat Application

#### Chujing Yu, Minghao Ma, Yiyang Liu

## Motivation

Real-time communication tools have become indispensable in recent years, reshaping how people connect and collaborate. Apps like WeChat, Discord, and Microsoft Teams have provided users with instant messaging, file sharing, and collaborative spaces.  

However, many of these popular tools are built using languages like JavaScript, C++, or Java, which, while effective, have limitations in terms of performance and memory management. Rust, in constrast, offers several advantages that could enhance the performance and reliability of real-time communication applications. Know for its memory safety, zero-cost abstractions, and high concurrency support, Rust provides a powerful foundation for building systems where performance is crucial. Its strict compile-time checks and efficient handling of resources help reduce the risk of runtime errors and ensure smoother performance even under heavy loads.

This project is motivated by our desire to leverage Rust's strengths to create a fast, scalable, and reliable real-time chat application. We hope to contribute to the relatively untapped potential of Rust in the field of real-time communication, where few open-source projects are readily available.

## Objective and key features

## Tentative plan

### 1. Real-time messaging using WebSockets

1. **WebSocket Integration**

2. **Scalability Considerations**

### 2. Chat room creation and joining

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
