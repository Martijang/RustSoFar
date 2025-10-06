
use std::net::{TcpStream, ToSocketAddrs};
use std::env;
use std::process::exit;
use std::time::Duration;

fn send_request(address: &String, port: i32) -> Result<Duration, Box<dyn std::error::Error>>{
    let now = std::time::Instant::now();

    let urls = format!("{}:{}", address, port);
    let connect = TcpStream::connect(&urls)?;
    connect.set_ttl(117)?;
    let resolved_addr:Vec<_> = urls.to_socket_addrs()?.collect();
    let duration = now.elapsed();

    println!("\tResponse of {} address:{:?}, time={:?} TTL={:?}", 
    address, 
    resolved_addr,
    duration,
    connect.ttl()?
);
    std::thread::sleep(std::time::Duration::from_secs(1));

    Ok(duration)
}

fn get_average(args: &String) -> Result<Duration, Box<dyn std::error::Error>>{
    let mut count = 5;
    let mut average: Duration = Duration::ZERO;
        loop {
            if count == 0 {
                break;
            }
            let value = send_request(&args, 80)?;
            average += value;
            count -= 1;
        }
        Ok(average / 5)
}

fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() >= 2 {
        match get_average(&args[1]){
            Ok(duration) => println!("For {}    \n Average is: {:?}", &args[1], duration), 
            Err(e) => eprintln!("Some thing went wrong: {:?}", e),
        };
    }else{
        println!("Usage: ./pingrs.exe target port");
        exit(0)
    }
}
