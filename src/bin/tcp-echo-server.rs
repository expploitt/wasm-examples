use std::net::{Shutdown, TcpStream};
use std::io::{Write, Read};
#[cfg(target_os = "wasi")]
use std::os::wasi::io::FromRawFd;
#[cfg(target_os = "linux")]
use std::os::unix::io::FromRawFd;


fn get_fd_listener() -> Option<std::net::TcpListener> {
    let stdlistener = unsafe { std::net::TcpListener::from_raw_fd(3) };
    stdlistener.set_nonblocking(true).unwrap();
    match stdlistener.local_addr() {
        Ok(a) => println!(" $ nc {} {}", a.ip(), a.port()),
        Err(_) => println!(" $ nc <IP> <PORT>"),
    }

    stdlistener.set_nonblocking(true).unwrap();

    Some(stdlistener)
}

fn main() {
    let listener = get_fd_listener().unwrap();


    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => {
                println!("Something went wrong: {}", e.to_string());
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf: [u8; 1024] = [0; 1024];

    println!("> New incoming connection");

    stream.write(b"> Welcome to Wasm Echo TCP Server! If you want to close the connection, please write 'exit'.\n").unwrap();
    stream.read(&mut buf).unwrap();

    while match stream.read(&mut buf) {
        Ok(size) => {
            stream.write(&buf[0..size]).unwrap();
            true
        }
        Err(e) => {
            println!("> Connection closed by {}", e.to_string());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}