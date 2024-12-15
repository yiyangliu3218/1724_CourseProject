/*Check online/offline status */
use crate::state::SharedState;
use tokio_tungstenite::connect_async;
use tungstenite::protocol::Message;
use futures_util::sink::SinkExt;        // 导入 SinkExt trait，用于 send 方法
use futures_util::stream::StreamExt;    // 导入 StreamExt trait，用于 next 方法
use actix_ws::AggregatedMessage;
use actix::prelude::*;

pub fn check_status(state: SharedState, user_id: &str) -> Result<String, String> {
    let users = state.users.lock().unwrap();
    // println!("{:?}",users);
    let userid = user_id.trim().to_string();
    // println!("{}",userid);
    // println!("{}",userid.len());
    // println!("{}",userid==String::from("user1"));
    if let Some(user) = users.get(&userid) {
        return Ok(if user.online {
            // println!("Online");
            format!("User {} is Online", user_id)
        } else {
            // println!("Offline");
            format!("User {} is Offline", user_id)
        });
    }else{
        println!("Failed to get.");
    }
    Err("User ID not found.".into())
}

pub async fn send_message(state: SharedState, receiver: &str, message: &str) -> Result<String, String>{
    let users = state.users.lock().unwrap();
    let receiver1 = receiver.trim().to_string();
    if let Some(user) = users.get(&receiver1) {
        return Ok(if user.online {
            let url = format!("ws://{}", user.address);
            let mut target = user.session.clone();
            match target.text(message).await {
                Ok(()) => {
                    println!("Message sent successfully.");
                    format!("Message sent successfully: {}", message)
                },
                Err(e) => {
                    println!("Failed to send due to error: {}", e);
                    format!("{}", message)
                },
            }
            // target.text(message.to_string()).await.unwrap();
            // 建立 WebSocket 连接
            /*
            match connect_async(&url).await {
                Ok((mut ws_stream, _)) => {
                // 向客户端发送消息
                    println!("Connected with client");
                    if let Err(e) = ws_stream.send(Message::Text(message.to_string())).await {
                        println!("Failed to send message: {}", e);
                    }
                }
                Err(e) => {
                    println!("Failed to connect to WebSocket client at {}: {}", url, e);
                }
            }
            */
        } else {
            println!("User is Offline");
            format!("User {} is Offline", receiver)
        });
    }
    else{
        println!("Cannot find user");
    }
    Err("User ID not found.".into())    
}

