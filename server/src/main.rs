use std::borrow::Cow;
use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
use std::sync::Arc;
use tokio::sync::{Mutex as AsyncMutex, Mutex};


#[derive(Debug)]
struct Client{
    addr: SocketAddr,
    stream : TcpStream,
    connected: bool,
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
            connected : self.connected
        }
    }
}

fn auth(credentials: Cow<str>) -> bool {
    let trimmed = credentials.trim_matches('$');
    let mut parts = trimmed.split(':');

    // Utilisez match pour déballer les valeurs de parts.next() et comparer
    match (parts.next(), parts.next()) {
        (Some("polo"), Some("mdp")) => true,
        _ => false,
    }
}


async fn handle_client(mut stream: TcpStream, clients: Arc<AsyncMutex<Vec<Client>>>) {
    let addr = stream.peer_addr().unwrap();
    let me = Client {
        addr,
        stream: stream.try_clone().expect("clone failed..."), // Note: Cloning stream for async TCPStream needs reevaluation
        connected: true,
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
                    println!("Connection closed by client {}", addr);
                    let mut clients_guard = clients.lock().await;
                    clients_guard.retain(|client| client.addr != addr);
                    return;
                }
                let credentials = String::from_utf8_lossy(&response[..bytes_read]);
                if auth(credentials.into()) {
                    println!("Authentication successful {}", addr);
                    stream.write_all(b" OK \n").expect("Failed to send success message");
                    break;
                } else {
                    println!("Authentication failed {}", addr);
                    stream.write_all(b" KO \n").expect("Failed to send success message");
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
        let _ = stream.shutdown(Shutdown::Both);
        return;
    }

    // Main message loop
    loop {
        let mut buffer = vec![0; 1024];
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    println!("Connection closed by client {}", addr);
                    break;
                }
                let message = String::from_utf8_lossy(&buffer[..bytes_read]);
                if !message.is_empty() {
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
        println!("New connection: {:?}", clients);
        tokio::task::spawn(async move {
            handle_client(stream, clients).await;
        });
    }
}