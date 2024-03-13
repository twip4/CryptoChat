use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

async fn handle_client(mut stream : TcpStream){
    let mut buffer = [0 ; 1024];
    stream.read(&mut buffer).expect("buffer error");
    let message = String::from_utf8_lossy(&buffer);
    println!("message: {}",message);
    let reponse = b"Hello you are connected !";
    stream.write(reponse).expect("TODO: panic message");
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming(){
            match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                handle_client(stream).await;
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }


drop(listener);
}