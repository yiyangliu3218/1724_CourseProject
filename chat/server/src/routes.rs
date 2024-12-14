use actix_web::web;
use actix_web::{rt, HttpRequest, HttpResponse};

use actix_ws::AggregatedMessage;
use futures_util::StreamExt as _; // For stream handling

use crate::services::auth;
use crate::services::chat;
use crate::services::message;
use crate::state::SharedState;

fn extract_word(s: &String, n: usize) -> &str {
    let mut j: usize = 1;
    let mut start: usize = 0;
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if j == n {
                return &s[start..i];
            } else if j == 2 && n == 4 {
                return &s[i + 1..];
            } else {
                j += 1;
                start = i + 1;
            }
        } else if i == s.len() - 1 {
            return &s[start..i + 1];
        }
    }
    &s[..]
}

// deal with all input commands
async fn echo(
    state: web::Data<SharedState>,
    req: HttpRequest,
    stream: web::Payload,
) -> HttpResponse {
    println!("New req received");
    let (res, mut session, stream) = actix_ws::handle(&req, stream).unwrap();
    session.text("Welcome! Please login with `login <user_id> <password>` or register with `register <user_id> <password>`...").await.unwrap();
    let mut connecter = String::new(); // the current connecter
    let mut stream = stream
        .aggregate_continuations()
        // aggregate continuation frames up to 1MiB
        .max_continuation_size(2_usize.pow(20));

    // start task but don't wait for it
    rt::spawn(async move {
        // receive messages from websocket
        while let Some(msg) = stream.next().await {
            println!("Connecting with: {}", connecter);
            let mut res_text = String::new();
            match msg {
                Ok(AggregatedMessage::Text(text)) => {
                    let received_text = text.clone().to_string();
                    let instruct = extract_word(&received_text, 1).to_string();

                    // deal with different instructions based on the first word
                    if instruct == String::from("register") {
                        if connecter != String::new() {
                            // cannot register if a user has logged in
                            res_text = String::from(
                                "Please log out of your current account before registeration.",
                            );
                        } else {
                            let id = extract_word(&received_text, 2).to_string();
                            let password = extract_word(&received_text, 3).to_string();
                            match auth::register(
                                session.clone(),
                                state.get_ref().clone(),
                                &id,
                                &password,
                            ) {
                                Ok(msg) => {
                                    res_text = msg.clone();
                                }
                                Err(err) => {
                                    res_text = err.clone();
                                }
                            }
                        }
                    } else if instruct == String::from("login") {
                        if connecter != String::new() {
                            res_text = String::from(
                                "Please log out of your current account before logging in.",
                            );
                        } else {
                            let id = extract_word(&received_text, 2).to_string();
                            let password = extract_word(&received_text, 3).to_string();
                            match auth::login(
                                session.clone(),
                                state.get_ref().clone(),
                                &id,
                                &password,
                            ) {
                                Ok(msg) => {
                                    res_text = msg.clone();
                                    connecter = id.clone(); // Update connecter
                                }
                                Err(err) => {
                                    res_text = err.clone();
                                }
                            }
                        }
                    } else if instruct == String::from("logout") {
                        if connecter == String::new() {
                            res_text = String::from("Please login first.");
                        } else {
                            let id = connecter.clone();
                            match auth::logout(state.get_ref().clone(), &id) {
                                Ok(msg) => {
                                    res_text = msg.clone();
                                    connecter = String::new(); // Update connecter
                                }
                                Err(err) => {
                                    res_text = err.clone();
                                }
                            }
                        }
                    } else if instruct == String::from("checkstatus") {
                        let id = extract_word(&received_text, 2).to_string();
                        match auth::check_status(state.get_ref().clone(), &id) {
                            Ok(msg) => {
                                res_text = msg.clone();
                            }
                            Err(err) => {
                                res_text = err.clone();
                            }
                        }
                    } else if instruct == String::from("privatechat") {
                        if connecter == String::new() {
                            res_text = String::from("Please login first.");
                        } else {
                            let sender = connecter.clone();
                            let receiver = extract_word(&received_text, 2).to_string();
                            let message = extract_word(&received_text, 4).to_string();
                            match message::private_chat(
                                state.get_ref().clone(),
                                &receiver,
                                &message,
                                &sender,
                            )
                            .await
                            {
                                Ok(inf) => {
                                    res_text = inf.clone();
                                }
                                Err(err) => {
                                    res_text = err.clone();
                                }
                            }
                        }
                    } else if instruct == String::from("createchatroom") {
                        if connecter == String::new() {
                            res_text = String::from("Please login first.");
                        } else {
                            let room_id = extract_word(&received_text, 2).to_string();
                            let max_capacity = extract_word(&received_text, 3).to_string();
                            if let Ok(capacity) = max_capacity.parse::<usize>() {
                                let result1 = chat::create_chatroom(
                                    state.get_ref().clone(),
                                    &room_id.to_string(),
                                    capacity,
                                );
                                match result1 {
                                    Ok(msg) => {
                                        res_text = msg.clone();
                                    }
                                    Err(err) => {
                                        res_text = err.clone();
                                    }
                                }
                                // after create a new chatroom, user join it.
                                let user_id = connecter.clone();
                                let result2 = chat::join_chatroom(
                                    state.get_ref().clone(),
                                    &room_id.to_string(),
                                    &user_id.to_string(),
                                );
                                res_text.push_str("\n");
                                match result2 {
                                    Ok(msg) => {
                                        res_text.push_str(&msg.clone());
                                    }
                                    Err(err) => {
                                        res_text.push_str(&err.clone());
                                    }
                                }
                            } else {
                                res_text = String::from(
                                    "Please enter an positive integer for chatroom capacity!",
                                );
                            }
                        }
                    } else if instruct == String::from("joinchatroom") {
                        if connecter == String::new() {
                            res_text = String::from("Please login first.");
                        } else {
                            let user_id = connecter.clone();
                            let room_id = extract_word(&received_text, 2).to_string();
                            let result = chat::join_chatroom(
                                state.get_ref().clone(),
                                &room_id.to_string(),
                                &user_id.to_string(),
                            );
                            match result {
                                Ok(msg) => {
                                    res_text = msg.clone();
                                }
                                Err(err) => {
                                    res_text = err.clone();
                                }
                            }
                        }
                    } else if instruct == String::from("leavechatroom") {
                        if connecter == String::new() {
                            res_text = String::from("Please login first.");
                        } else {
                            let user_id = connecter.clone();
                            let room_id = extract_word(&received_text, 2).to_string();
                            let result = chat::leave_chatroom(
                                state.get_ref().clone(),
                                &room_id.to_string(),
                                &user_id.to_string(),
                            );
                            match result {
                                Ok(msg) => {
                                    res_text = msg.clone();
                                }
                                Err(err) => {
                                    res_text = err.clone();
                                }
                            }
                        }
                    } else if instruct == String::from("listchatroom") {
                        if connecter == String::new() {
                            res_text = String::from("Please login first.");
                        } else {
                            let chatrooms = chat::list_chatrooms(state.get_ref().clone());
                            if chatrooms.is_empty() {
                                res_text = String::from("There is no chatroom.");
                            } else {
                                let room_list = chatrooms.join(", ");
                                res_text = format!("Available chatrooms: {}", room_list);
                            }
                        }
                    } else if instruct == String::from("sendmessage") {
                        if connecter == String::new() {
                            res_text = String::from("Please login first.");
                        } else {
                            let sender = connecter.clone();
                            let target_room = extract_word(&received_text, 2).to_string();
                            let message = extract_word(&received_text, 4).to_string();
                            match message::send_message(
                                state.get_ref().clone(),
                                &target_room,
                                &message,
                                &sender,
                            )
                            .await
                            {
                                Ok(inf) => {
                                    res_text = inf.clone();
                                }
                                Err(err) => {
                                    res_text = err.clone();
                                }
                            }
                        }
                    } else if instruct == String::from("listusers") {
                        if connecter == String::new() {
                            res_text = String::from("Please login first.");
                        } else {
                            let room_id = extract_word(&received_text, 2).to_string();
                            let result =
                                chat::list_users(state.get_ref().clone(), &room_id.to_string());
                            match result {
                                Ok(users) => {
                                    if users.is_empty() {
                                        res_text = format!(
                                            "There is no user in the chatroom {}",
                                            room_id.to_string()
                                        );
                                    } else {
                                        let user_list = users.join(",");
                                        res_text = format!(
                                            "Users in the chatroom {}: {}",
                                            room_id.to_string(),
                                            user_list
                                        );
                                    }
                                }
                                Err(err) => {
                                    res_text = err;
                                }
                            }
                        }
                    } else if instruct == String::from("quit") {
                        if connecter != String::new() {
                            let user_id = connecter.clone();
                            let result =
                                auth::logout(state.get_ref().clone(), &user_id.to_string());
                            match result {
                                Ok(msg) => {
                                    res_text = msg.clone();
                                }
                                Err(err) => {
                                    res_text = err.clone();
                                }
                            }
                        }
                        res_text = format!("{}\nGoodbye!", res_text);
                        session.text(res_text).await.unwrap();
                        session.close(None).await.unwrap();
                        break;
                    } else {
                        res_text = String::from("Invalid command!");
                    }
                    session.text(res_text).await.unwrap();
                }
                _ => (),
            };
        }
    });
    return res;
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/chat").route(web::get().to(echo)));
}
