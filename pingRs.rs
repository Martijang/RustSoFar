
use std::net::ToSocketAddrs;
use std::env;
use std::process::exit;
use std::time::Duration;

pub fn send_request(address: String, port: i32) -> Result<Duration, Box<dyn std::error::Error>>{
    let now = std::time::Instant::now();
    let urls = format!("{}:{}", address, port);
    let addrs:Vec<_> = urls.to_socket_addrs()?.collect();
    let duration = now.elapsed();

    println!("\tResponse of {} address:{:?}, time={:?}", address, addrs, duration);
    std::thread::sleep(std::time::Duration::from_secs(1));

    Ok(duration)
}

fn get_average(args: Vec<String>) -> Duration{
    let mut count = 5;
    let mut average: Duration = Duration::ZERO;
        loop {
            if count == 0 {
                break;
            }
            let value = send_request(args[1].clone(), 80);
            average += value.expect("ERROR");
            count -= 1;
        }
        return average / 5;
}
fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() >= 2 {
        println!("for {} \n      Average is: {:?}",&args.clone()[1], get_average(args));
    }else{
        println!("Usage: ./pingrs.exe target port");
        exit(0)
    }
}
