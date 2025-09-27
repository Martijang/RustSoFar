use std::io::{Read, Write};
use std::net::{TcpListener,TcpStream};
use std::fs;
use std::env;
use std::process::exit;
use std::thread;

fn responed_as_file(mut stream: TcpStream, path: &String) ->Result<(), std::io::Error>{
    let mut buffer = [0u8; 512];

    stream.read(&mut buffer)?;

    let file = fs::read_to_string(path)?;
    let get = b"GET / HTTP/1.1\r\n";
    if buffer.starts_with(get) {
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            file.len(),
            file
     ); 
        stream.write(response.as_bytes())?;
        println!("Successfully responded");
    }else {
        //if any invaild request happens, send ERROR 404
        stream.write(b"HTTP/1.1 404 NOT FOUND\r\n\r\n <h1>404</h1> <p>NOT FOUND</p>")?;
        println!("404 code sent");
    }

    Ok(())
}

fn main() -> std::io::Result<()>{
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2{
        println!("Usage: ./<exe> <your_file_to_send> <Your IP:Port>");
        exit(0);
    }

    let path = &args[1];
    let listener = TcpListener::bind(&args[2])?;

    for stream in listener.incoming(){
        let responder = responed_as_file(stream.expect("ERRPR"), &path);

        thread::spawn(move || {
          responder.expect("ERROR");
        }).join().expect("Cannot join");
    }

    Ok(())
}
