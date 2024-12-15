use crate::handlers::{auth_handler, presence_handler};
use actix_web::web;
use actix_web::{rt, App, Error, HttpRequest, HttpResponse, HttpServer};

use actix_ws::AggregatedMessage;
use actix::prelude::*;
use futures_util::StreamExt as _; // For stream handling
use std::sync::{Arc, Mutex};

use crate::services::presence_service;
use crate::services::auth_service;
use crate::state::SharedState;

// 提取第n个单词,n=4则会返回除了前两个单词以外的整句话
fn extract_word(s: &String, n: usize) -> &str {
    let mut j:usize = 1;
    let mut start:usize = 0;
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            if j==n {
                // println!("{}",&s[start..i]);
                return &s[start..i];
            }else if (j==2 && n==4){
                return &s[i+1..];
            }else{
                j += 1;
                start = i+1;
            }
        }
        else if i==s.len()-1{
            // println!("{}",&s[start..i]);
            return &s[start..i+1];
        }
    }
    &s[..]
}

// 统一处理所有输入
async fn echo(state: web::Data<SharedState>, req: HttpRequest, stream: web::Payload) -> HttpResponse {
    println!("New req received");
    let (mut res, mut session, stream) = actix_ws::handle(&req, stream).unwrap();
    let mut success = false;
    // let mut ret = Arc::new(Mutex::new(res));
    let mut stream = stream
        .aggregate_continuations()
        // aggregate continuation frames up to 1MiB
        .max_continuation_size(2_usize.pow(20));

    // start task but don't wait for it
    rt::spawn(async move{
        // receive messages from websocket
        //let ret_clone = Arc::clone(&ret);
        
        while let Some(msg) = stream.next().await {
            let mut res_text = String::new();
            let resp = match msg {
                Ok(AggregatedMessage::Text(text)) => {
                    // echo text message
                    success = true;
                    let received_text = text.clone().to_string();
                    let instruct = extract_word(&received_text,1).to_string(); // 转换为 String
                    // println!("{}",received_text);
                    // 根据第一个单词处理不同的输入情况
                    let Response1 = {
                    if instruct == String::from("register"){
                        let id = extract_word(&received_text,2).to_string();
                        
                        let password = extract_word(&received_text,3).to_string();
                        // println!("{}",password.len());
                        match auth_service::register(session.clone(), state.get_ref().clone(), &id, &password) {
                            Ok(msg) => {
                                // println!("Successfully registered");
                                res_text = String::from("Successfully registered");
                                HttpResponse::Ok().body(msg)
                            },
                            Err(err) => HttpResponse::BadRequest().body(err),
                        }
                    }else if instruct == String::from("login"){
                        let id = extract_word(&received_text,2).to_string();
                        let password = extract_word(&received_text,3).to_string();
                        match auth_service::login(session.clone(), &req,state.get_ref().clone(), &id, &password) {
                            Ok(msg) => {
                                // println!("Successfully login");
                                res_text = String::from("Successfully login");
                                HttpResponse::Ok().body(msg)
                            },
                            Err(err) => {
                                println!("failed");
                                HttpResponse::BadRequest().body(err)
                            },
                        }
                    }else if instruct == String::from("logout"){
                        let id = extract_word(&received_text,2).to_string();
                        match auth_service::logout(state.get_ref().clone(), &id) {
                            Ok(msg) => {
                                res_text = String::from("Successfully logout");
                                HttpResponse::Ok().body(msg)
                            },
                            Err(err) => HttpResponse::BadRequest().body(err),
                        }
                    }else if instruct == String::from("checkstatus"){
                        let id = extract_word(&received_text,2).to_string();
                        // println!("{}",id);
                        match presence_service::check_status(state.get_ref().clone(), &id) {
                            Ok(msg) => {
                                // println!("{}", id);
                                res_text = msg.clone();
                                HttpResponse::Ok().body(msg)
                            },
                            Err(err) => {
                                // println!("{}Err", id);
                                HttpResponse::BadRequest().body(err)
                            },
                        }
                    }else if instruct == String::from("sendmessage"){
                        let receiver = extract_word(&received_text,2).to_string();
                        let message = extract_word(&received_text,4).to_string();
                        println!("{}",message);
                        match presence_service::send_message(state.get_ref().clone(), &receiver, &message).await{
                            Ok(inf) => {
                                res_text = inf.clone();
                                // println!("send succeed");
                                HttpResponse::Ok().body(inf)
                            },
                            Err(err) => {
                                println!("Send failed due to error: {}",err);
                                HttpResponse::BadGateway().body(err)
                            },
                        }
                    }else{
                        res_text = String::from("Invalid instruction!");
                        HttpResponse::BadRequest().body("Invalid instruction.")
                    }
                    }; 
                    session.text(res_text).await.unwrap();
                    Response1
                    //let ret = ret_clone.lock().unwrap();
                    //let mut value = ret.lock().unwrap();
                    //*value = Response1; // 更新共享变量                  
                    // 
                }
                _ => HttpResponse::Ok().body("status"),
            };
            //return resp;
        }
     
    });
    // let s = HttpResponse::Ok().body("OK");
    // return HttpResponse::Ok().body("OK");
    return res;
    // respond immediately with response connected to WS session
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/logout").route(web::post().to(auth_handler::logout)))
        .service(
            web::resource("/checkstatus").route(web::post().to(presence_handler::check_status)),
        ).service(web::resource("/sendmessage").route(web::get().to(presence_handler::send_message)))
        .service(web::resource("/echo").route(web::get().to(echo)));
        // .service(web::resource("/register").route(web::post().to(auth_handler::register)));
        //.service(web::resource("/login").route(web::post().to(auth_handler::login)))
}
