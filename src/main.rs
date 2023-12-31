
use std::{
    fs,
    io::{self, prelude::*, BufReader},
    net::{TcpListener, TcpStream}, 
    thread,
    time::Duration,
    sync::{Arc, atomic::{AtomicBool, Ordering}},
};

use hello::ThreadPool;

const IP_ADDR: &str = "127.0.0.1:7878";

fn main() {
    let listener = TcpListener::bind(IP_ADDR).unwrap();
    let pool = ThreadPool::build(8).unwrap();
    let exit = Arc::new(AtomicBool::new(false)); 
    let exit_clone = Arc::clone(&exit);

    pool.execute(move || {
        let quit = "quit";
        let mut input = String::from("");
        let stdin = io::stdin();

        println!("Enter 'quit' to shutdown the server");

        while quit != input {
            input = stdin.lock().lines().next().unwrap().unwrap();
        }

        exit_clone.store(true, Ordering::Relaxed);
    });

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });

        if exit.load(Ordering::Relaxed) {
            break;
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    const CRLF: &str = "\r\n";

    let (status_line, file_name) = match &request_line[..] {
         "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
    
    let contents = fs::read_to_string(file_name).unwrap();
    let len = contents.len();
    
    let response = 
        format!("{status_line}{CRLF}Content-Length: {len}{CRLF}{CRLF}{contents}");

    stream.write_all(response.as_bytes()).unwrap();

}
