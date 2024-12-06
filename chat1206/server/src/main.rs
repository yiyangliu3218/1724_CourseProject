use crate::services::{auth, chat};
use crate::state::{AppState, SharedState};
use actix::ActorContext;
use actix::{Actor, Running};
use actix_rt;
use actix_web::web::get;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio_tungstenite::tungstenite::protocol::Message;

mod models;
mod services;
mod state;

// handle incoming websocket connection requests
async fn websocket_handler(
    req: HttpRequest,
    stream: web::Payload,
    state: web::Data<SharedState>,
) -> Result<HttpResponse, Error> {
    ws::start(
        WebSocketSession {
            user_id: None,
            state: state.get_ref().clone(),
        },
        &req,
        stream,
    )
}

// websocket session
struct WebSocketSession {
    user_id: Option<String>, // user id associated with this session
    state: SharedState,
}

impl actix::Actor for WebSocketSession {
    type Context = ws::WebsocketContext<Self>;
    // when start, send a message prompting the user to authenticate
    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.text("Welcome! Please login with `login <user_id> <password>` or register with `register <user_id> <password>`...");
    }
    // when stop(disconnect), remove the user's session from session map if authenticated
    fn stopping(&mut self, _: &mut Self::Context) -> actix::Running {
        // remove session on disconnect
        // if let Some(user_id) = &self.user_id {
        //     let mut state = self.state.sessions.lock().unwrap();
        //     state.retain(|_, id| id != user_id);
        // }
        actix::Running::Stop
    }
}

// handle incoming websocket messages
impl actix::StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => self.handle_message(text.to_string(), ctx),
            Ok(ws::Message::Close(reason)) => ctx.close(reason),
            _ => (),
        }
    }
}

impl WebSocketSession {
    fn handle_message(&mut self, text: String, ctx: &mut ws::WebsocketContext<Self>) {
        let state = self.state.clone();
        let parts: Vec<&str> = text.split_whitespace().collect();

        match parts.as_slice() {
            ["register", user_id, password] => {
                if let Some(user_id) = &self.user_id {
                    ctx.text("Please log out of your current account before registeration.");
                } else {
                    let result = auth::register(state, &user_id.to_string(), &password.to_string());
                    match result {
                        Ok(msg) => ctx.text(msg),
                        Err(err) => ctx.text(err),
                    }
                }
            }
            ["login", user_id, password] => {
                if let Some(user_id) = &self.user_id {
                    ctx.text("Please log out of your current account before logging in.");
                } else {
                    let result = auth::login(state, &user_id.to_string(), &password.to_string());
                    match result {
                        Ok(msg) => {
                            ctx.text(msg);
                            self.user_id = Some(user_id.to_string());
                        }
                        Err(err) => ctx.text(err),
                    }
                }
            }
            ["logout"] => {
                if let Some(user_id) = &self.user_id {
                    let result = auth::logout(state, &user_id.to_string());
                    match result {
                        Ok(msg) => {
                            ctx.text(msg);
                            self.user_id = None;
                        }
                        Err(err) => ctx.text(err),
                    }
                } else {
                    ctx.text("Please login first.");
                }
            }
            ["checkstatus", user_id] => {
                let result = auth::check_status(state, &user_id.to_string());
                match result {
                    Ok(msg) => ctx.text(msg),
                    Err(err) => ctx.text(err),
                }
            }
            ["createchatroom", room_id, max_capacity] => {
                if let Some(user_id) = &self.user_id {
                    if let Ok(capacity) = max_capacity.parse::<usize>() {
                        let result1 =
                            chat::create_chatroom(state.clone(), &room_id.to_string(), capacity);
                        match result1 {
                            Ok(msg) => ctx.text(msg),
                            Err(err) => ctx.text(err),
                        }
                        // after create a new chatroom, user join it.
                        let result2 =
                            chat::join_chatroom(state, &room_id.to_string(), &user_id.to_string());
                        match result2 {
                            Ok(msg) => ctx.text(msg),
                            Err(err) => ctx.text(err),
                        }
                    } else {
                        ctx.text("Please enter an positive integer for chatroom capacity!");
                    }
                } else {
                    ctx.text("Please login first.");
                }
            }
            ["joinchatroom", room_id] => {
                if let Some(user_id) = &self.user_id {
                    let result =
                        chat::join_chatroom(state, &room_id.to_string(), &user_id.to_string());
                    match result {
                        Ok(msg) => ctx.text(msg),
                        Err(err) => ctx.text(err),
                    }
                } else {
                    ctx.text("Please login first.");
                }
            }
            ["leavechatroom", room_id] => {
                if let Some(user_id) = &self.user_id {
                    let result =
                        chat::leave_chatroom(state, &room_id.to_string(), &user_id.to_string());
                    match result {
                        Ok(msg) => ctx.text(msg),
                        Err(err) => ctx.text(err),
                    }
                } else {
                    ctx.text("Please login first.");
                }
            }
            ["listchatroom"] => {
                if let Some(user_id) = &self.user_id {
                    let chatrooms = chat::list_chatrooms(state);
                    if chatrooms.is_empty() {
                        ctx.text("There is no chatroom.");
                    } else {
                        let room_list = chatrooms.join(", ");
                        ctx.text(format!("Available chatrooms: {}", room_list));
                    }
                } else {
                    ctx.text("Please login first.");
                }
            }
            ["listusers", room_id] => {
                if let Some(user_id) = &self.user_id {
                    let result = chat::list_users(state, &room_id.to_string());
                    match result {
                        Ok(users) => {
                            if users.is_empty() {
                                ctx.text(format!(
                                    "There is no user in the chatroom {}",
                                    room_id.to_string()
                                ));
                            } else {
                                let user_list = users.join(",");
                                ctx.text(format!(
                                    "Users in the chatroom {}: {}",
                                    room_id.to_string(),
                                    user_list
                                ));
                            }
                        }
                        Err(err) => ctx.text(err),
                    }
                } else {
                    ctx.text("Please login first.");
                }
            }
            ["quit"] => {
                if let Some(user_id) = &self.user_id {
                    let result = auth::logout(state, &user_id.to_string());
                    match result {
                        Ok(msg) => ctx.text(msg),
                        Err(err) => ctx.text(err),
                    }
                }
                ctx.text("Goodbye!");
                ctx.close(Some(ws::CloseReason {
                    code: ws::CloseCode::Normal,
                    description: Some("User requested disconnect".to_string()),
                }));
                ctx.stop(); // Ensure session stops after sending close frame
            }

            _ => ctx.text("Unknown command"),
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState::new());

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/ws", web::get().to(websocket_handler))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}

// WebSocket message handler
// impl ws::StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
//     fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
//         match msg {
//             Ok(ws::Message::Text(text)) => {
//                 let state = self.state.clone();
//                 let addr = self.addr;
//                 let parts: Vec<&str> = text.split_whitespace().collect();

//                 match parts.as_slice() {
//                     ["register", user_id, password] => {
//                         let user_id = user_id.to_string();
//                         let password = password.to_string();
//                         actix::spawn(async move {
//                             handle_register(state, addr, user_id, password).await;
//                         });
//                     }
//                     ["login", user_id, password] => {
//                         let user_id = user_id.to_string();
//                         let password = password.to_string();
//                         actix::spawn(async move {
//                             handle_login(state, addr, user_id, password).await;
//                         });
//                     }
//                     ["logout"] => {
//                         actix::spawn(async move {
//                             handle_logout(state, addr).await;
//                         });
//                     }
//                     ["checkstatus"] => {
//                         actix::spawn(async move {
//                             handle_presence(state, addr).await;
//                         });
//                     }
//                     ["createroom", room_id, capacity] => {
//                         let room_id = room_id.to_string();
//                         let capacity = capacity.to_string();
//                         actix::spawn(async move {
//                             handle_createroom(state, addr, room_id, capacity).await;
//                         });
//                     }
//                     _ => println!("Unknown command: {}", text),
//                 }
//             }
//             Ok(ws::Message::Close(reason)) => {
//                 ctx.close(reason);
//                 ctx.stop();
//             }
//             _ => {}
//         }
//     }
// }

// // handle WebSocket connection
// async fn ws_handler(req:HttpRequest,stream:web::Payload,stste::web::Data<AppState>)->Result<HttpResponse,Error>{
//     let addr=req.peer_addr().unwrap();
//     println!("New WebSocket connection from: {}", addr);

//     // user login, then build a websocket session
//     let user_id
// }
