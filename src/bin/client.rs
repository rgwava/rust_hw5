use lazy_static::lazy_static;
use std::net::SocketAddr;
use miniredis::LogLayer;
use miniredis::FilterLayer;

use std::io::{self};

lazy_static! {
    static ref CLIENT: volo_gen::miniredis::ItemServiceClient = {
        let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        volo_gen::miniredis::ItemServiceClientBuilder::new("miniredis")
            .layer_outer(LogLayer)
            .layer_outer(FilterLayer)
            .address(addr)
            .build()
    };
}
#[volo::main]
async fn main() {
    
    loop{
        println!("-------------------------------");
        println!("mini-redis is working:");
        println!("input 'wq' to exit:");
        let mut input = String::new();
        if let Ok(_) = io::stdin().read_line(&mut input) {
            println!("Received line: {}", input.trim());
        } else {
            println!("No input received");
            continue;
        }
        let words:Vec<String> = input.trim().split(' ').map(|s| s.to_string()).collect();
        //println!("{:?}",words);
        let mut req = volo_gen::miniredis::GetItemRequest { tyep: 0, key: " ".into(), value: " ".into() };
        
    match words[0].as_str(){
        "wq"=>{
            break;
        }
        "get"=>{
            println!("you have input command 'get'");
            match words.len(){
                1=>{println!("not enough input");continue;}
                2=>{
                    req.tyep=0;
                    req.key = words[1].clone().into();
                }
                _=>{println!("too much input");continue;}
            }
        }
        "set"=>{
            println!("you have input command 'set'");
            match words.len(){
                1=>{println!("not enough input");continue;}
                2=>{println!("not enough input");continue;}
                3=>{
                    req.tyep = 1;
                    req.key = words[1].clone().into();
                    req.value = words[2].clone().into();
                }
                _=>{println!("too much input");continue;}
            }

        }
        "del"=>{
            println!("you have input command 'del'");
            match words.len(){
                1=>{println!("not enough input");continue;}
                2=>{
                    req.tyep = 2;
                    req.key = words[1].clone().into();
                }
                _=>{println!("too much input");continue;}
            }
            
        }
        "ping"=>{
            println!("you have input command 'ping'");
            match words.len(){
                1=>{
                    req.tyep = 3;
                }
                _=>{
                    println!("too much input");
                    continue;
                }
            }
        }
        _=>{
            println!("wrong command");
            continue;
        }
    }
        println!("type:{} key:{} value:{}",req.tyep,req.key,req.value);
        println!("#");
        let resp = CLIENT.get_item(req).await;
        println!("#");
        match resp {
           
            Ok(info) => {
                if info.tyep == 0 {
                    println!("0");
                    if info.success {
                        println!("the value is:{}", info.value);
                    }
                    else {
                        println!("'{}' is a wrong key",info.key);
                    }
                }
                if info.tyep == 1 {
                    println!("1");
                    if info.success {
                        println!("set the key '{}' successflly",info.key);
                    } else {
                        println!("key '{}' exists already,and it was reassigned ",info.key);
                    }
                }
                if info.tyep == 2 {
                    println!("2");
                    if info.success {
                        println!("deleted successfully");
                    }
                    else {
                        println!("the key '{}' did not exist",info.key);
                    }
                }
                if info.tyep == 3 {
                    println!("3");
                    if info.success {
                        println!("pong");
                    } else {
                        println!("ping failed");
                    }
                }
            },
            Err(err) => {println!("err!");tracing::error!("{:?}", err)},
        }

        println!("end");

    }
}   