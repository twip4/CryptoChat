use std::io;
use std::io::{Read, Write};
use std::net::{TcpStream};

struct User {
    login: String,
    password: String,
}

fn connexion() -> User{

    let mut tmp_login= String::new();
    let mut tmp_pasword = String::new();

    println!("Please enter your login :");

    io::stdin()
        .read_line(&mut tmp_login)
        .expect("Failed to read line");

    println!("Please enter your password :");

    io::stdin()
        .read_line(&mut tmp_pasword)
        .expect("Failed to read line");

    let user = User {
        login: tmp_login,
        password: tmp_pasword
    };

    return user
}

fn main() {
    println!("Welcome to Crypto Chat");

    let user = connexion();

    let mut stream = TcpStream::connect("127.0.0.1:7878").expect("error connextion");
    println!("Successfully connected to server Crypto Chat");
    let msg = user.login.as_bytes();
    stream.write(msg).expect("TODO: panic message");
    let msg = user.password.as_bytes();
    stream.write(msg).expect("TODO: panic message");

    loop {
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                // Only consider the part of the buffer that contains data
                let buffer = &buffer[..bytes_read];
                let message = String::from_utf8_lossy(buffer);
                if !message.is_empty() {
                    println!("{}", message);
                }
            },
            Err(e) => {
                println!("Failed to receive data: {}", e);
            }
        }
    }
}