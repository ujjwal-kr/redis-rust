use std::sync::{Arc, Mutex};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

mod commands;
mod decoder;
mod encoder;
mod storage;

use decoder::*;
use encoder::*;
use storage::Storage;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    let storage_engine = Arc::new(Mutex::new(Storage::new()));
    println!("Listening on ::6379");
    loop {
        let incoming = listener.accept().await;
        let cloned_storage = Arc::clone(&storage_engine);
        match incoming {
            Ok((mut stream, _)) => {
                println!("New Connection");
                tokio::spawn(async move {
                    handle_connection(&mut stream, cloned_storage).await;
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

async fn handle_connection(stream: &mut TcpStream, client_store: Arc<Mutex<Storage>>) {
    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf).await.unwrap();
        // break the loop if no bytes recieved
        if bytes_read == 0 {
            println!("Client closed the connection");
            break;
        }
        let str_cmd = String::from_utf8_lossy(&buf);
        let cmd: Vec<&str> = str_cmd.split("\r\n").collect::<Vec<&str>>();
        if cmd[0].len() != 2 {
            stream
                .write(&encode_resp_error_string("(error) Cannot Process"))
                .await
                .unwrap();
        }
        let cmd_len: usize = cmd[0][1..2].parse::<usize>().unwrap() * 2;
        let pure_cmd = decode_get_pure_command(cmd[0..cmd_len + 1].to_vec());

        match pure_cmd[0].to_ascii_lowercase().trim() {
            "ping" => commands::ping(stream).await,
            "echo" => commands::echo(stream, pure_cmd).await,
            "set" => commands::set(stream, pure_cmd, Arc::clone(&client_store)).await,
            "get" => commands::get(stream, pure_cmd, Arc::clone(&client_store)).await,
            "del" => commands::del(stream, pure_cmd, Arc::clone(&client_store)).await,
            "lpush" | "rpush" => commands::push(stream, pure_cmd, Arc::clone(&client_store)).await,
            "lrange" => commands::lrange(stream, pure_cmd, Arc::clone(&client_store)).await,
            _ => commands::undefined(stream).await,
        };
    }
}
