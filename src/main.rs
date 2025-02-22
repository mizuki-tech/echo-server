use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::{thread, env, str};
use std::result::Result;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let port = &args[1];
    echo_server(port)?;

    Ok(())
}

fn echo_server(port: &str) -> Result<(), Box<dyn Error>> {
    println!("Hi!1");
    let listener = TcpListener::bind(port)?;
    println!("Hi!2");
    loop {
        println!("Hi!3");
        let (mut stream, _) = listener.accept()?;
        println!("Hi!4");
        thread::spawn(move || {
            let mut buffer = [0u8; 1024];
            loop {
                let nbytes = stream.read(&mut buffer).expect("Read error");
                if nbytes == 0 {
                    println!("Hi!5");
                    return;
                }
                print!("{}", str::from_utf8(&buffer[..nbytes]).expect("Error converting to string"));
                stream.write_all(&buffer[..nbytes]).expect("Write error");
            }
        });
    }
}
