use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::sink::SinkExt;        // 导入 SinkExt trait，用于 send 方法
use futures_util::stream::StreamExt;    // 导入 StreamExt trait，用于 next 方法
use std::fmt;
// use std::io;
use std::sync::{Arc};
use actix_web::{rt, App, Error, HttpRequest, HttpResponse, HttpServer};
use tokio::task;
use tokio::sync::Mutex; // 使用 tokio 的 Mutex
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::runtime;

#[tokio::main]
async fn main() {
    // WebSocket 服务器地址
    let url = "ws://127.0.0.1:8081/echo";
    let mut cookie:String = String::new();

    // 连接到 WebSocket 服务器
    let (mut ws_stream, _) = connect_async(url)
        .await
        .expect("Failed to connect");

    // 启动一个任务来处理接收消息
    // let ws_stream = Arc::new(Mutex::new(ws_stream));
    // let ws_stream_clone = Arc::clone(&ws_stream);
    println!("Client opened");

    // 拆分 WebSocket 流为接收器（reader）和发送器（writer）
    let (mut writer, mut reader) = ws_stream.split();

    task::spawn(async move {
        // let mut stream = ws_stream_clone.lock().await;  
        while let Some(message) = reader.next().await {
            // println!("debug");
            match message {
                Ok(Message::Text(response)) => {
                    println!("{}", response);
                }
                Err(e) => {
                    eprintln!("Error receiving message: {}", e);
                    break;
                }
                _=>()
            }
        }
    });
        
        // 处理用户输入并发送消息
        let stdin = io::stdin();
        let reader = BufReader::new(stdin);
        let mut lines = reader.lines();
    
        while let Ok(Some(line)) = lines.next_line().await {
            let mut input = line.trim().to_string();
            // println!("{}",input);
            if input.is_empty() {
                continue; // 忽略空输入
            }
        // input = cookie + " " + &input[..];
    
            // let mut stream = ws_stream.lock().await;
            // println!("Sending");
            match writer.send(Message::Text(input)).await {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Error sending message: {}", e);
                    break;
                }
            }
        }
    /*
    while(true){
        /*
        tokio::select! {
            // 处理用户输入
            _ = async {
                let mut input = String::new();
                println!("Input");
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                let mut stream = ws_stream.lock().await;
                println!("Send");
                stream.send(Message::Text(input.trim().into())).await.expect("Failed to send message");
            } => {},
        }
        */
        io::stdout().flush();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let mut stream2 = ws_stream.lock().unwrap();
        stream2
            .send(Message::Text(input.into()))
            .await
            .expect("Failed to send message");
    }
    */
        // println!("Send");
    /*
    // 发送一个文本消息
    io::stdout().flush();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let mut stream2 = ws_stream.lock().unwrap();
    stream2
        .send(Message::Text(input.into()))
        .await
        .expect("Failed to send message");
    
    // println!("send success");
    // 等待并接收服务器的响应
     
    if let Some(Ok(Message::Text(response))) = ws_stream.next().await {
        println!("{}", response);  // 应该收到
    }else{
        println!("Error");
    }
    */   
}
