use reqwest::Client;
use std::io::{self, Write};

pub async fn start_cli() {
    let client = Client::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let args: Vec<&str> = input.trim().split_whitespace().collect();
        if args.is_empty() {
            continue;
        }

        match args[0] {
            "register" => {
                if args.len() == 3 {
                    let user_id = args[1];
                    let password = args[2];
                    handle_register(&client, user_id, password).await;
                } else {
                    println!("Usage: register <user ID> <password>");
                }
            }
            "login" => {
                if args.len() == 3 {
                    let user_id = args[1];
                    let password = args[2];
                    handle_login(&client, user_id, password).await;
                } else {
                    println!("Usage: login <user ID> <password>");
                }
            }
            "createchatroom" => {
                if args.len() == 3 {
                    let room_id = args[1];
                    let capacity: usize = args[2].parse().unwrap_or(0);
                    handle_create_chatroom(&client, room_id, capacity).await;
                } else {
                    println!("Usage: createchatroom <room ID> <capacity>");
                }
            }
            "joinchatroom" => {
                if args.len() == 3 {
                    let room_id = args[1];
                    let user_id = args[2];
                    handle_join_chatroom(&client, room_id, user_id).await;
                } else {
                    println!("Usage: joinchatroom <room ID> <user ID>");
                }
            }
            "leavechatroom" => {
                if args.len() == 3 {
                    let room_id = args[1];
                    let user_id = args[2];
                    handle_leave_chatroom(&client, room_id, user_id).await;
                } else {
                    println!("Usage: leavechatroom <room ID> <user ID>");
                }
            }
            "listchatrooms" => {
                handle_list_chatrooms(&client).await;
            }
            "listusers" => {
                if args.len() == 2 {
                    let room_id = args[1];
                    handle_list_users(&client, room_id).await;
                } else {
                    println!("Usage: listusers <room ID>");
                }
            }
            _ => println!("Unknown command: {}", args[0]),
        }
    }
}

async fn handle_register(client: &Client, user_id: &str, password: &str) {
    let body = serde_json::json!({
        "user_id": user_id,
        "password": password,
    });
    let res = client
        .post("http://127.0.0.1:8081/auth/register")
        .json(&body)
        .send()
        .await;
    match res {
        Ok(response) => println!("{}", response.text().await.unwrap()),
        Err(err) => eprintln!("Error: {}", err),
    }
}

async fn handle_login(client: &Client, user_id: &str, password: &str) {
    let body = serde_json::json!({
        "user_id": user_id,
        "password": password,
    });
    let res = client
        .post("http://127.0.0.1:8081/auth/login")
        .json(&body)
        .send()
        .await;
    match res {
        Ok(response) => println!("{}", response.text().await.unwrap()),
        Err(err) => eprintln!("Error: {}", err),
    }
}

async fn handle_create_chatroom(client: &Client, room_id: &str, capacity: usize) {
    let body = serde_json::json!({
        "room_id": room_id,
        "capacity": capacity,
    });
    let res = client
        .post("http://127.0.0.1:8081/chatroom/create")
        .json(&body)
        .send()
        .await;
    match res {
        Ok(response) => println!("{}", response.text().await.unwrap()),
        Err(err) => eprintln!("Error: {}", err),
    }
}

async fn handle_join_chatroom(client: &Client, room_id: &str, user_id: &str) {
    let body = serde_json::json!({
        "room_id": room_id,
        "user_id": user_id,
    });
    let res = client
        .post("http://127.0.0.1:8081/chatroom/join")
        .json(&body)
        .send()
        .await;
    match res {
        Ok(response) => println!("{}", response.text().await.unwrap()),
        Err(err) => eprintln!("Error: {}", err),
    }
}

async fn handle_leave_chatroom(client: &Client, room_id: &str, user_id: &str) {
    let body = serde_json::json!({
        "room_id": room_id,
        "user_id": user_id,
    });
    let res = client
        .post("http://127.0.0.1:8081/chatroom/leave")
        .json(&body)
        .send()
        .await;
    match res {
        Ok(response) => println!("{}", response.text().await.unwrap()),
        Err(err) => eprintln!("Error: {}", err),
    }
}

async fn handle_list_chatrooms(client: &Client) {
    let res = client
        .get("http://127.0.0.1:8081/chatroom/list")
        .send()
        .await;
    match res {
        Ok(response) => println!("{}", response.text().await.unwrap()),
        Err(err) => eprintln!("Error: {}", err),
    }
}

async fn handle_list_users(client: &Client, room_id: &str) {
    let res = client
        .get(&format!("http://127.0.0.1:8081/chatroom/{}/users", room_id))
        .send()
        .await;
    match res {
        Ok(response) => println!("{}", response.text().await.unwrap()),
        Err(err) => eprintln!("Error: {}", err),
    }
}


// // 只有login和reg

// use reqwest::Client;

// use std::io::{self, Write};

// pub async fn start_cli() {
//     let client = Client::new();

//     loop {
//         print!("> ");
//         io::stdout().flush().unwrap();
//         let mut input = String::new();
//         io::stdin().read_line(&mut input).unwrap();

//         let args: Vec<&str> = input.trim().split_whitespace().collect();
//         if args.is_empty() {
//             continue;
//         }

//         match args[0] {
//             "register" => {
//                 if args.len() == 3 {
//                     let user_id = args[1];
//                     let password = args[2];
//                     handle_register(&client, user_id, password).await;
//                 } else {
//                     println!("Usage: register <user ID> <password>");
//                 }
//             }
//             "login" => {
//                 if args.len() == 3 {
//                     let user_id = args[1];
//                     let password = args[2];
//                     handle_login(&client, user_id, password).await;
//                 } else {
//                     println!("Usage: login <user ID> <password>");
//                 }
//             }
//             _ => println!("Unknown command: {}", args[0]),
//         }
//     }
// }

// async fn handle_register(client: &Client, user_id: &str, password: &str) {
//     let body = serde_json::json!({
//         "user_id": user_id,
//         "password": password,
//     });
//     let res = client
//         .post("http://127.0.0.1:8081/auth/register")
//         .json(&body)
//         .send()
//         .await;
//     match res {
//         Ok(response) => println!("{}", response.text().await.unwrap()),
//         Err(err) => eprintln!("Error: {}", err),
//     }
// }

// async fn handle_login(client: &Client, user_id: &str, password: &str) {
//     let body = serde_json::json!({
//         "user_id": user_id,
//         "password": password,
//     });
//     let res = client
//         .post("http://127.0.0.1:8081/auth/login")
//         .json(&body)
//         .send()
//         .await;
//     match res {
//         Ok(response) => println!("{}", response.text().await.unwrap()),
//         Err(err) => eprintln!("Error: {}", err),
//     }
// }
