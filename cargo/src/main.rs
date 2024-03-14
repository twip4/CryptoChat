use std::process;
use tokio::io::{self, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::select;

struct User {
    login: String,
    password: String,
}

fn connexion() -> User {
    let mut tmp_login = String::new();

    println!("Please enter your login:");
    std::io::stdin()
        .read_line(&mut tmp_login)
        .expect("Failed to read line");
    tmp_login = tmp_login.trim().to_string();

    println!("Please enter your password:");
    let tmp_password = rpassword::read_password().expect("Failed to read password");


    User {
        login: tmp_login,
        password: tmp_password,
    }
}


#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Welcome to Crypto Chat");

    let mut user: User;

    let mut stream = TcpStream::connect("127.0.0.1:7878").await.expect("error connection");

    println!("Successfully connected to server in Crypto Chat");

    loop {

        user = connexion();

        let credentials = format!("${}:{}$", user.login, user.password);
        stream.write_all(credentials.as_bytes()).await.expect("Failed to send credentials");

        let mut response = [0; 1024];
        let bytes_read = stream.read(&mut response).await.expect("Failed to read from stream");

        if bytes_read == 0 {
            println!("La connexion a été fermée par le serveur.");
            process::exit(0);
        }

        let message = String::from_utf8_lossy(&response[..bytes_read]);

        if message.trim() == "OK" {
            println!("Authentification réussie.");
            break;
        }
        else if message.trim() == "ERROR" {
            println!("Échec de l'authentification.");
        }
        else {
            println!("Connexion fermée.");
            stream.shutdown().await?;
            process::exit(0);
        }
    }


    let mut stdin = BufReader::new(io::stdin());
    let mut buffer = vec![0; 1024];
    let mut stdin_buffer = String::new();

    loop {
        select! {
            bytes_read = stream.read(&mut buffer) => {
                let bytes_read = bytes_read?;
                if bytes_read == 0 {
                    println!("Server closed the connection.");
                    break;
                }
                println!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
            },
            read_result = stdin.read_line(&mut stdin_buffer) => {
                let bytes_read = read_result?;
                if bytes_read == 0 {
                    println!("EOF on stdin, stopping.");
                    break;
                }
                if !stdin_buffer.trim().is_empty() {
                    stream.write_all(stdin_buffer.as_bytes()).await?;
                    stdin_buffer.clear();
                }
            },
        }
    }

    Ok(())
}
