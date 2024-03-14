mod treatment;

use std::borrow::Cow;
use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
use std::sync::Arc;
use tokio::sync::{Mutex as AsyncMutex, Mutex};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
struct Users {
    users: Vec<User>,
}

#[derive(Serialize, Deserialize)]
struct User {
    login: String,
    password: String,
}

#[derive(Debug)]
struct Client{
    addr: SocketAddr,
    stream : TcpStream,
    connected: bool,
    pseudo: String,
}

impl PartialEq for Client {
    fn eq(&self, other: &Self) -> bool {
        self.addr == other.addr
    }
}

impl Clone for Client {
    fn clone(&self) -> Self {
        Client{
            addr : self.addr.clone(),
            stream: self.stream.try_clone().expect("clone failed..."),
            connected : self.connected,
            pseudo : self.pseudo.clone(),
        }
    }
}

fn auth(credentials: Cow<str>) -> bool {
    let trimmed = credentials.trim_matches('$');
    let parts: Vec<&str> = trimmed.split(':').collect();

    // Lecture et désérialisation du fichier JSON
    let file_content = fs::read_to_string("data/users.json").expect("Failed to read users file");
    let users: Users = serde_json::from_str(&file_content).expect("Failed to parse JSON");

    // Vérification des identifiants
    users.users.iter().any(|user| {
        parts.get(0) == Some(&user.login.as_str()) && parts.get(1) == Some(&user.password.as_str())
    })
}


async fn handle_client(mut stream: TcpStream, clients: Arc<AsyncMutex<Vec<Client>>>) {
    let addr = stream.peer_addr().unwrap();
    let mut me = Client {
        addr,
        stream: stream.try_clone().expect("clone failed..."), // Note: Cloning stream for async TCPStream needs reevaluation
        connected: true,
        pseudo: String::new(),
    };
    {
        let mut clients = clients.lock().await;
        clients.push(me.clone());
    }

    let mut count_trial = 0;
    while count_trial < 3 {
        let mut response = vec![0; 1024];
        match stream.read(&mut response) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    println!("Connection closed by clientdzdz {}", addr);
                    let mut clients_guard = clients.lock().await;
                    clients_guard.retain(|client| client.addr != addr);
                    return;
                }
                let credentials = String::from_utf8_lossy(&response[..bytes_read]);
                if auth(credentials.clone().into()) {
                    println!("Authentication successful {}", addr);
                    stream.write_all(b" OK \n").expect("Failed to send success message");
                    let trimmed = credentials.trim_matches('$');
                    let mut parts = trimmed.split(':');
                    if let Some(pseudo) = parts.next() {
                        me.pseudo = pseudo.to_string();
                    } else {
                        println!("Pseudo manquant dans les identifiants");
                        me.pseudo = "Pseudo Error".parse().unwrap();
                    }
                    break;
                } else {
                    println!("Authentication failed {}", addr);
                    stream.write_all(b" ERROR \n").expect("Failed to send success message");
                }
            }
            Err(_) => {
                println!("Failed to read credentials from {}", addr);
                return;
            }
        }
        count_trial += 1;
    }

    if count_trial == 3 {
        println!("Connection closed due to failed authentication {}", addr);
        stream.write_all(b" KO \n").expect("Failed to send success message");
        let mut clients_guard = clients.lock().await;
        clients_guard.retain(|client| client.addr != addr);
        let _ = stream.shutdown(Shutdown::Both);
        return;
    }

    // Main message loop
    loop {
        let mut buffer = vec![0; 1024];
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    println!("Connection closed by clientdzdz {}", addr);
                    break;
                }
                let mut message = String::from_utf8_lossy(&buffer[..bytes_read]);
                if !message.is_empty() {
                    message = Cow::from(treatment::treatment::analyse(message.to_string(), me.clone()));
                    broadcast_write(me.clone(), clients.clone(), message.to_string()).await;
                    println!("Message from {}: {}", addr, message);
                }
            },
            Err(e) => {
                println!("Failed to receive data from {}: {}", addr, e);
                break;
            }
        }
    }
    let mut clients_guard = clients.lock().await;
    clients_guard.retain(|client| client.addr != addr);
}

async fn broadcast_write(emeteur: Client, clients: Arc<Mutex<Vec<Client>>>, msg: String) {
    let mut clients = clients.lock().await;
    for client in &mut *clients {
        if client.addr != emeteur.addr {
            client.stream.write(msg.as_bytes()).expect("TODO: panic message");
        }
    }
}
#[tokio::main]
async fn main() {
    let clients = Arc::new(AsyncMutex::new(Vec::new()));

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    loop {
        let (stream, _addr) = match listener.accept() {
            Ok((stream, addr)) => (stream, addr),
            Err(e) => {
                eprintln!("Error: {}", e);
                continue;
            }
        };

        let clients = Arc::clone(&clients);
        // println!("New connection: {:?}", clients);
        tokio::task::spawn(async move {
            handle_client(stream, clients).await;
        });
    }
}