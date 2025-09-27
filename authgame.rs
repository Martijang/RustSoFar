//rpassword version 7.4.0

use std::collections::HashMap;
use std::io::{self, stdout, Write};
use std::process::exit;

use rpassword::read_password;

pub struct Auth{
    map: HashMap<String, String>,
}

impl Auth{
    pub fn new() -> Self{
        Auth { map: HashMap::new() }
    }
    
    pub fn create_account(&mut self) {
        print!("name~# ");
        let name = get_input();
        print!("password~# ");
        let _ = stdout().flush();
        let password = read_password().expect("Cannot read the password");

        if self.map.contains_key(&name) {
            println!("Username already exists");
        } else {
            self.map.insert(name, password);
            println!("Account created");
        }
    }

    pub fn delete_account(&mut self) {
        print!("name~# ");
        let name = get_input();
        print!("password~# ");
        let _= stdout().flush();

        let password = read_password().expect("Cannot read the password");

        if let Some(value) = self.map.get(&name) {
            if value == &password {
                self.map.remove(&name);
                println!("Account removed");
            } else {
                println!("Username or password is invalid");
            }
        } else {
            println!("Username or password is invalid");
        }
    }

    pub fn login(&mut self) {
        print!("name~# ");
        let name = get_input();
        print!("password~# ");
        let _= stdout().flush();
        let password = read_password().expect("Cannot read password");

        if let Some(value) = self.map.get(&name) {
            if value == &password {
                println!("Welcome in!");
            } else {
                println!("Username or password is invalid");
        }
        } else {
            println!("Username or password is invalid");
        }
    }
}

fn main() {
    println!("Menu: | create acc | delete acc | login | exit");
    let mut auth = Auth::new();

    loop {
        print!("system@code~# ");
        let input = get_input();
        let trimed = input.trim();

        match trimed {
            "create acc" => Auth::create_account(&mut auth),
            "delete acc" => Auth::delete_account(&mut auth),
            "login" => Auth::login(&mut auth),
            "exit" => exit(0),
            _ => println!("Invalid command. Please try again."),
        }
    }
}

fn get_input() -> String {
    let _ = stdout().flush();
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("RUN TIME ERROR: cannot get input");

    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }
    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }

    return input;
}

