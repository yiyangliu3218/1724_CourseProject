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
            // "sendmessage" => {
            //     if args.len() >= 3 {
            //         let room_id = args[1];
            //         let message = args[2..].join(" ");
            //         handle_send_message(&client, room_id, &message).await;
            //     } else {
            //         println!("Usage: sendmessage <room ID> <message>");
            //     }
            // }
            // "quit" => {
            //     println!("Exiting...");
            //     break;
            // }
            _ => println!("Unknown command: {}", args[0]),
        }
    }
}

async fn handle_register(client: &Client, user_id: &str, password: &str) {
    let body = serde_json::json!({ "user_id": user_id, "password": password });
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
    let body = serde_json::json!({ "user_id": user_id, "password": password });
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
    let body = serde_json::json!({ "room_id": room_id, "capacity": capacity });
    let res = client
        .post("http://127.0.0.1:8081/chatroom")
        .json(&body)
        .send()
        .await;
    match res {
        Ok(response) => println!("{}", response.text().await.unwrap()),
        Err(err) => eprintln!("Error: {}", err),
    }
}

// async fn handle_send_message(client: &Client, room_id: &str, message: &str) {
//     let body = serde_json::json!({ "room_id": room_id, "message": message });
//     let res = client
//         .post("http://127.0.0.1:8080/chatroom/message")
//         .json(&body)
//         .send()
//         .await;
//     match res {
//         Ok(response) => println!("{}", response.text().await.unwrap()),
//         Err(err) => eprintln!("Error: {}", err),
//     }
// }
