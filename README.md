# 1724_CourseProject
### Commands from the User

* `register <use ID> <password>` : Register a new user account
* `login <use ID> <password>` : Log into the user account
* `logout`: Exit the user account
* `checkstatus <user ID>` : Presence detection to show the online\offline status with the given user id
* `createchatroom <room ID> <room capacity>` : Create a new char room with specific capacity and join it
* `joinchatroom <room ID>`: Join the chatroom with the given room id
* `leavechatroom <room ID>` : Leave the chatroom with the given room id
* `listchatroom <room ID>` : List the ids of all chatrooms the user is currently in
* `listusers <room ID>` : List all the users in the chatroom with the given room id.
* `sendmessage <room ID> <message>` : Send a message to the chatroom with the given room id.
* `quit` : Exit the server and terminate the program

chat_app/
├── Cargo.toml               # Project dependencies and metadata
├── src/
│   ├── main.rs              # Entry point of the application
│   ├── config.rs            # Configuration setup (e.g., port, database URL)
│   ├── routes.rs            # HTTP/WebSocket routes
│   ├── handlers/
│   │   ├── auth.rs          # Handlers for authentication (register, login, logout)
│   │   ├── chat.rs          # Handlers for chat room actions (create, join, list)
│   │   ├── message.rs       # Handlers for sending and receiving messages
│   │   └── presence.rs      # Handlers for presence detection
│   ├── models/
│   │   ├── user.rs          # User-related structures and logic
│   │   ├── chat_room.rs     # Chat room-related structures and logic
│   │   └── message.rs       # Message-related structures
│   ├── services/
│   │   ├── websocket.rs     # WebSocket implementation
│   │   ├── auth_service.rs  # Business logic for authentication
│   │   ├── chat_service.rs  # Business logic for chat room management
│   │   └── presence_service.rs  # Business logic for presence detection
│   ├── utils.rs             # Utility functions (e.g., password hashing)
│   └── state.rs             # Shared application state (e.g., users, rooms)
└── README.md                # Project description and instructions
