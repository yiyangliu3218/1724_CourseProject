# ECE1724 Project: Real-Time Chat Application

Chujing Yu, 1010809608, <chujing.yu@mail.utoronto.ca>\
Minghao Ma, 1011818428, <minghao.ma@mail.utoronto.ca>\
Yiyang Liu, 1011770512, <yyoung.liu@mail.utoronto.ca>

## Video Demo

<https://youtu.be/cweYx_vz5fc>

## Github Branch Overview

* `main`:  Contains the final, up-to-date version of the project's code.
* `archive`: Preserves previous versions of the project. For **individual contributions**, refer to the commit history of this branch.

## Motivation

Real-time communication tools have become indispensable in recent years, transforming how people connect and collaborate. Popular applications like WeChat, Discord, and Microsoft Teams offer features such as instant messaging, file sharing, and collaborative spaces.

However, these widely used tools are often built using languages like JavaScript, C++, or Java. While effective, these languages present limitations, particularly in performance and memory management, as they often struggle with high concurrency demands and real-time responsiveness under heavy loads.

Rust, on the other hand, is a systems programming language known for its memory safety, zero-cost abstractions, and high concurrency support, making it particularly well-suited for building high-performance, reliable real-time communication applications. Rust’s strict compile-time checks, efficient resource handling, and robust error handling provide significant advantages, reducing the risk of runtime errors and ensuring smoother performance even when the system is under pressure.

This project is motivated by our desire to leverage Rust's strengths to create a fast, scalable, and reliable real-time chat application. By doing so, we aim to contribute to the Rust ecosystem, which currently lacks open-source real-time communication solutions, and to explore the untapped potential of Rust in this field.

## Objective and key features

In this project, we aim to develop a real-time chat application using Rust that is capable of handling multiple users and high concurrency while maintaining stability and security. This application will include several core functions:

* **User Authentication and Presence Detection**: Users will be able to register, log in, and log out with their unique accounts. Their online status will be updated in real time, ensuring that other users in shared chat rooms are aware of who is active or inactive.
* **Chat Room Management**: Users can create or join chat rooms with unique IDs, and they can participate in multiple chat rooms simultaneously. The system will handle HTTP requests from users to manage chat room operations and efficiently track each room’s active users.
* **Real-Time Messaging with WebSockets**: Users in the same chat room can send and receive text messages in real time, enabled by a WebSocket server for message routing and broadcast. This server will also handle connection management, message filtering, and other essential tasks.
* **Scalability and Security**: The application will leverage Rust's concurrency and memory safety features to ensure that it remains robust and efficient even under high concurrency, enhancing the reliability of real-time communications.

## Features

### Basic User Authentication

The project implements robust user authentication, ensuring secure access control through following key functionalities:

#### 1. Registration

The registration process allows users to create an account with a unique user ID and a password. The `bcrypt` crate is used to hash the password before storing it, ensuring that sensitive information is never stored in plaintext. The registration process performs the following steps:

* **Status Check**: Ensure the user has not logged in an account.
* **Validation**: Checks if the provided user ID already exists in the HashMap `users`.
* **Password Hashing**: The `bcrypt::hash` function hashes the password with a computational cost factor of 4.
* **User Struct Creation**: A `User` struct is instantiated with the following variables:
  * `id`: The user's unique identifier.
  * `password`: The hashed password.
  * `online`: A boolean indicating the user's online status, initially set to `false`.
  * `session`: The WebSocket session for real-time communication.

* **Insertion**: The new user is inserted into the shared HashMap `users` protected by a `Mutex`.

**Command**: `register <user id> <password>`
**Response**: Returns a success message or an error if the registration fails.

#### 2. Log in

The login process authenticates users based on their credentials. Key steps include:

* **Status Check**: Ensure the user has not logged in an account.
* **Credential Verification**: Checks if the input user id exists and uses the `bcrypt::verify` function to compare the provided password with the stored hashed password.
* **Online Status Update**: If credentials are valid and the user is not already online, the variable `online` is set to `true`.
* **Session Management**: The user's WebSocket `session` is updated to enable real-time communication.

**Command**: `login <user_id> <password>`
**Response**: Success message upon login or error for incorrect credentials or duplicate login attempts.

#### 3. Log out

The logout function updates the user's status to offline and clears their session information:

* **Status Check**: Ensure the user has logged in an account.
* **Status Reset**: Sets `online` to `false`.
* **Session Cleanup**: Disassociates the user from their current WebSocket session.

**Command**: `logout`
**Response**: Confirms successful logout or returns an error if the user is already logged out.

#### 4. Security Considerations

* **Session Management**: The system maintains an active WebSocket `session` for each logged-in user, enabling real-time messaging and presence updates. Sessions are updated during login and cleared during logout to avoid stale connections.
* **Concurrency Safety**: The HashMap `users` in the struct `SharedState` are protected by a `Mutex`, ensuring that concurrent access from multiple WebSocket connections does not lead to race conditions.

### Chat Room Management

#### 1. Chat Room Creation

Users can dynamically create chat rooms, providing a structured space for group conversations:

* **Validation**: Ensures that the room name is unique and capacity is a positive integer within allowed limits.
* **Room Struct Creation**: Initializes a Room struct with variables:
  * `name`: The chatroom's unique identifier.
  * `capacity`: Maximum number of users allowed in the room.
  * `users`: A list to track participants in real-time.
* **Insertion**: Adds the new room to the HashMap `chatrooms` within `SharedState`, protected by a `Mutex` for concurrency safety.

**Command**: `createchatroom <room name> <capacity>`
**Response**: Returns a success message or an error if the room name already exists or input parameters are invalid.

#### 2. Joining Chat Rooms

Users can join existing chat rooms, fostering collaborative communication:

* **Existence Check**: Verifies the specified chat room exists in the system.
* **Capacity Check**: Ensures the room is not full.
* **User Addition**: Adds the user's ID to the room's `users` list if conditions are met.

**Command**: `joinchatroom <room_id>`
**Response**: Success message upon joining or an error if the room is full or does not exist.

#### 3. Leaving Chat Rooms

Users can exit chat rooms to manage their participation dynamically:

* **Membership Check**: Verifies if the user is a participant in the specified room.
* **User Removal**: Removes the user's ID from the room's `users` list.

**Command**: `leavechatroom <room_id>`
**Response**: Confirmation of successful exit or an error for invalid requests.

#### 4. Listing Chat Rooms

Users can retrieve a list of all active chat rooms to explore available spaces.
**Command**: `listchatroom`
**Response**: Displays the names of all existing rooms.

#### 5. Listing Users in a Chat Room

Users can view the current participants of any specified chat room.
**Command**: `listusers <room_id>`
**Response**: Lists all active members in the specified chat room or returns an error if the room does not exist.

#### 6. Concurrency Safety

Shared state structures like `chatrooms` are protected by a `Mutex` to avoid race conditions during concurrent access.

By incorporating these functionalities, the system supports effective and secure management of chat rooms, enabling seamless and collaborative interactions.

### Real-Time Messaging

#### 1. Private Chat

The `privatechat` command allows users to send messages to a specific user. When a user connects to the server and logs in, the session information is extracted from the HTTP request and stored on the server. This session is associated with their unique user ID. This feature allows for one-on-one communication between users.

Key Technical Details:

* **Session Management:** The server maintains a session for each connected user, and stores other information based on their unique user ID.
* **Targeted Messaging:** The **Private Chat** command specifies a recipient user. The server verifies if the recipient is currently online by checking their session status.
* **Direct Communication:** If the recipient is online, the server sends the message directly to the recipient’s session. If the recipient is offline, the sender will receive an error message.

This feature enables direct communication between users in the same chat application, offering a more personal, private messaging experience.

**Command:** `privatechat <user> <message>`

#### 2. Message Broadcasting

The **Message Broadcasting** is similar to **Private Chat**. Instead of sending messages to a specific user, it allows users to broadcast to an entire chat room. They can send messages to everyone in a specific chat room.

Key Technical Details:

* **Session Management:** Like the `privatechat` command, the server retrieves and stores the sender's session information.
* **Message Broadcasting:** Upon receiving the `sendmessage` command, the server checks if the sender is part of the specified chat room. If the sender is a member of the room, the server retrieves the session details of all users in the target chat room.
* **Message Distribution:** The server then broadcasts the message to all users in the chat room by sending it through each user’s session. The message is relayed only to those who are currently online and connected to the chat room.

This approach ensures that the message is broadcast in real-time to all participants within the room while verifying the sender’s membership to the chat room for proper message routing.

**Command:** `sendmessage <room name> <message>`

### Presence Detection

Presence Detection allows users to check the real-time status of other users. This feature is implemented via the `check_status()` function, which queries the HashMap `users` for the specified user ID. Key steps include:

* **Locking State**: Accesses the shared `users` state by locking  the `Mutex` to ensure thread safety.
* **Status Check**: Returns a message indicating whether the user is online (`user.online==true`) or offline.
* **Error Handling**: Returns an error if the specified user ID does not exist in the users collection.

**Command**: `checkstatus <user id>`
**Response**:
`User <user id> is Online` if the user is online.
`User <user id> is offline` if the user is offline.
Error message if the user ID is not found.

### Command-Line Interface

When you launch the client application, it connects to the WebSocket server and displays a prompt where you can enter commands. Each command is followed by relevant arguments (e.g., user IDs, passwords, or messages). The client receives real-time feedback and messages from the server, making interactions seamless and responsive.

### Error Handling

|**Error**|**Error Message**|
|--------------|---------------|
|Invalid command|Invalid command!|
|Duplicate registration|User ID already exists.|
|Invalid password|Password verification failed.|
|Duplicate logging in|Please log out of your current account before logging in.|
|Invalid user ID|User ID not found.|
|Duplicate logging out|User has already logged out.|
|Duplicate chat room creation|Chatroom x already exists! |
|Invalid room capacity |Please enter a positive integer for chatroom capacity! |
|Chatroom is full |Chatroom x is full! |
|Duplicate chat room join |User n is already in chatroom x! |
|Invalid chat room ID |Chatroom x does not exist! |
|Not in the chat room |User m is not in chatroom x! |
|Send messages to an offline user |User m is Offline |
|Attempted to send message without joining the chat room |You have to join the room first to send message! |
|Attempted to register while logged in |Please log out of your current account before registration. |
|Not logged in while managing chat rooms or sending messages |Please login first. |

## User's Guide

The real-time chat application supports user authentication, presence detection, chat room management and real-time messaging. This guide provides instructions for using the features.

### Running the Application

**1. Starting the Server**
Navigate to the `server` directory and run the following command to launch the server:

```bash
> cd server
> cargo run
```

The server will start and listen on <http://127.0.0.1:8081> for WebSocket connections.

**2. Starting the Client**
Open a new terminal, navigate to the `client` directory, and run:

```bash
> cd client
> cargo run
```

This will connect the client to the WebSocket server at <ws://127.0.0.1:8081/chat>.

### Available Commands

#### 1. Authentication

* **Register a new user:**

```bash
> register <user_id> <password>
```

Example: `register Alice 12345`\
Success: `User Alice registered successfully.`

* **Log in:**

```bash
> login <user_id> <password>
```

Example: `login Alice 12345`\
Success: `User Alice logged in successfully.`

* **Log out:**

```bash
> logout
```

Success: `User Alice logged out successfully.`

#### 2. Presence Detection

* **Check user status:**

```bash
> checkstatus <user_id>
```

Example: `checkstatus Alice`\
Success: `User Alice is Online` or `User Alice is Offline.`

#### 3. Private Messaging

* **Send a private message:**

```bash
> privatechat <recipient_user_id> <message>
```

Example: `privatechat Tom Hello, Tom!`\
Note: Make sure that user Tom has registered and logged in in another client terminal.

#### 4. Chatroom Management

* **Create a chat room:**

```bash
> createchatroom <room_id> <max_capacity>
```

Example: `createchatroom room1 3`\
Success: `Chatroom room1 created with capacity 3! User Alice joined chatroom room1!`\
Note: Users must log in before creating a chat room; Users automatically join the chat room they created.

* **Join a chat room:**

```bash
> joinchatroom <room_id>
```

Example: `joinchatroom room1`\
Success: `User Tom joined chatroom room1!`\
Note: Users must log in before joining a chat room.

* **Leave a chat room:**

```bash
> leavechatroom <room_id>
```

Example: `leavechatroom room1`\
Success: `User Tom left chatroom room1!`

* **List available chat rooms:**

```bash
> listchatroom
```

Response: `Available chatrooms: room1`\
Note: Users must log in before asking for the chat room list.

* **List Users in a Chatroom:**

```bash
> listusers <room_id>
```

Example: `listusers room1`\
Success: `Users in the chatroom room1: Alice, Tom`

#### 5. Broadcast Messaging

* **Send a message to a chat room**

```bash
> sendmessage <room_id> <message>
```

Example: `sendmessage room1 Hello, everyone!`

#### 6. Quit the Client

```bash
> quit
```

Response: `Goodbye!` and close the connection.

## Reproducibility Guide

Follow these steps to set up, build, and run the chat application on Ubuntu Linux, macOS Sonoma, and Windows.

### Prerequisites

#### 1. Install System Dependencies

**For Ubuntu**
Install essential build tools and libraries:

```bash
> sudo apt update
> sudo apt install -y build-essential pkg-config libssl-dev curl
```

**For macOS**
Ensure you have Homebrew installed. If not, install it using:

```bash
> /bin/bash -c "$(curl -fsSL <https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh>)"
```

Then install OpenSSL:

```bash
> brew install openssl
```

**For Windows**
Ensure you have the Microsoft C++ Build Tools installed. You can install them via the Visual Studio Installer or download them directly from:
<https://visualstudio.microsoft.com/visual-cpp-build-tools/>
During installation, select the “Desktop development with C++” workload.

#### 2. Install Rust and Cargo

Rust and Cargo are required for all systems.

**For Ubuntu and macOS**

* Install rustup using the following command:

```bash

> curl --proto '=https' --tlsv1.2 -sSf <https://sh.rustup.rs> | sh
```

* Follow the prompts to complete the installation. After installation, load the Rust environment:

```bash
> source $HOME/.cargo/env
```

* Verify the installation:

```bash
> rustc --version
> cargo --version
```

**For Windows**

* Download and run the Rust installer from the official website:
<https://www.rust-lang.org/tools/install>
* During installation, ensure that the “Add to PATH” option is selected.
* After installation, open a new Command Prompt or PowerShell and verify the installation:

```bash
> rustc --version
> cargo --version
```

### Project Setup

#### 1. Directory Structure

```text
chat/
│
├── client/
│   ├── src/
│   │   └── main.rs     # Entry point of the client
│   └── Cargo.toml      # Manages dependencies and configuration for client
└── server/
    ├── src/
    │   ├── models/
    │   │   ├── chat_room.rs    # Chat room-related structures
    │   │   ├── user.rs     # User-related structures
    │   │   └── mod.rs
    │   │
    │   ├── services/
    │   │   ├── auth.rs     # Logic for authentication and presence detection
    │   │   ├── chat.rs     # Logic for chat room management
    │   │   ├── message.rs    # Logic for sending messages
    │   │   └── mod.rs
    │   │
    │   ├── main.rs     # Entry point of the server
    │   ├── routes.rs     # HTTP/WebSocket routes
    │   └── state.rs      # Shared application state (e.g., users, rooms)
    └── Cargo.toml      # Manages dependencies and configuration for server
```

#### 2. Clone the Project Repository

```bash
> git clone https://github.com/yiyangliu3218/1724_CourseProject.git
> cd chat
```

### Build the Server and Client

#### 1.Build the Server

Navigate to the `server` directory

```bash
> cd server
> cargo build
```

#### 2. Build the Client

Open a new terminal or command prompt and navigate to the `client` directory:

```bash
> cd client
> cargo build
```

### Run the Server and Client

#### 1. Start the Server

From the `server` directory, run:

```bash
> cargo run
```

The server will start listening on <http://127.0.0.1:8081>.

#### 2. Start the Client

In a separate terminal or command prompt, navigate to the `client` directory and run:

```bash
> cargo run
```

This will connect client to the server at <ws://127.0.0.1:8081/chat>.

### Testing the Application

#### Available Commands

`register <use_id> <password>` : Register a new user account\
`login <use_id> <password>` : Log into the user account\
`logout`: Exit the user account\
`checkstatus <user_id>` : Presence detection to show the online\offline status with the given user id\
`createchatroom <room_id> <room capacity>` : Create a new char room with specific capacity and join it\
`joinchatroom <room_id>` : Join the chatroom with the given room id\
`leavechatroom <room_id>` : Leave the chatroom with the given room id\
`listchatroom <room_id>` : List the ids of all chatrooms the user is currently in\
`listusers <room_id>` : List all the users in the chatroom with the given room id\
`privatechat <recipient_user_id> <message>` : Send a message to the user with the given user id\
`sendmessage <room_id> <message>` : Send a message to the chatroom with the given room id\
`quit` : Close the connection and quit the client

For detailed examples and explanations, refer to the **User's Guide**.

### Troubleshooting

#### Port Conflicts

If port `8081` is already in use, follow these steps to identify and close the process using the port:

**1. Find the process using port `8081`:**

**Ubuntu Linux:**

```bash
> sudo lsof -i :8081
```

**macOS:**

```bash
> lsof -i :8081
```

**Windows:**

```bash
> netstat -ano | findstr :8081
```

**2. Terminate the process:**
Based on the `PID` (process ID) from the previous step, run the following commands:

**Ubuntu Linux:**

```bash
> sudo kill -9 <PID>
```

**macOS:**

```bash
> kill -9 <PID>
```

**Windows:**

```bash
> taskkill /PID <PID> /F
```

**3. Verify the port is free:**

**Ubuntu Linux:**

```bash
> sudo lsof -i :8081
```

**macOS:**

```bash
> lsof -i :8081
```

**Windows:**

```bash
> netstat -ano | findstr :8081
```

If no output is returned, the port is now free.

## Contributions

<table>
<tr>
<th> Chujing Yu</th>
<th> Minghao Ma</th>
<th> Yiyang Liu</th>
</tr>
<tr>
<td align="center">Implemented core functions for User Authentication</td>
<td align="center">Implemented core functions for sending messages</td>
<td align="center">Implemented core functions for Chat Room Management</td>
</tr>
<tr>
<td align="center">Implemented core functions for Presence Detection</td>
<td align="center">Dealing with other issues related to communication</td>
<td align="center">Implemented core functions for User and Chat Room Listing Features</td>
</tr>
<tr>
<td colspan="2" align="center">Developed Command-Line Interface </td>
<td align="center">Completed video demo</td>
</tr>
<tr>
<td colspan="3" align="center">Testing and debugging</td>
</tr>
<tr>
<td colspan="3" align="center">Completed individual parts of project proposal and final report</td>
</tr>
</table>

## Lessons Learned and Concluding Remarks

In conclusion, our Real-Time Chat Application project has been an exciting and rewarding experience that allowed us to explore the power of Rust and Actix Web for building scalable, high-performance systems. Throughout the development process, we focused on delivering a real-time messaging platform with basic user authentication, chat room management, and presence detection features—all while ensuring smooth communication between users.

One of the key challenges we faced was managing WebSocket connections and ensuring seamless communication in real-time across multiple clients. However, by leveraging Rust’s concurrency features and Actix Web's robust support for asynchronous operations, we were able to implement a performant solution that could handle multiple users concurrently.

We hope that this application serves as a useful starting point for anyone looking to build real-time communication systems, and we are excited to continue exploring new possibilities in the world of Rust development.
