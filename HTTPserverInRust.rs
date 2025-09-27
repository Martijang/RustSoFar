use std::io::{Read, Write};
use std::net::{TcpListener,TcpStream};
use std::fs;
use std::env;
use std::thread;

fn responed_as_file(mut stream: TcpStream){
    let args: Vec<String> = env::args().collect();
//useage:
// eg: ./<exe> <your_file_to_send> <Your IP and Port> NOTE: make sure to direct a file properly, like: /User/home/index.html
//";
    let mut buffer = [0; 1024];

    let _=stream.read(&mut buffer).expect("cannot read a response");
    println!("{:?}", String::from_utf8_lossy(&buffer[..]));

    let file = fs::read_to_string(&args[1]).expect("file not found");
    let get = b"GET / HTTP/1.1\r\n";
    if buffer.starts_with(get) {
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            file.len(),
            file
     ); 
    //this is where the index.html file will be sent
    stream.write(
        response.as_bytes()
    ).expect("client has lost a connection or error while sending response. Please check your connection");
    }else {
        //if any invaild request happens, send ERROR 404
        let _ = stream.write(b"HTTP/1.1 404 NOT FOUND\r\n\r\n")
        .expect("client has lost a connection or error while sending response. Please check your connection");
        let _= stream.write(b"<h1>404</h1> <p>NOT FOUND</p>")
        .expect("client has lost a connection or error while sending response. Please check your connection");
    }

}

fn handle_client(stream: TcpStream){
    //some other things might might be here
    responed_as_file(stream);
}

fn main() -> std::io::Result<()>{
    let args: Vec<String> = env::args().collect();
    let listener = TcpListener::bind(&args[2]).unwrap();

    if args.len() > 2{
        println!("Usage: ./<exe> <your_file_to_send> <Your IP:Port>");
    }

    for stream in listener.incoming(){
        let stream = stream.unwrap();

        thread::spawn(move || {
          handle_client(stream);
        });
    }

    Ok(())

}
