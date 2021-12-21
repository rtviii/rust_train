use std::fs;
use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}




fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    let get   = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get){
        ("HTTP/1.1 200 OK", "hello.html")
    } 
    else if buffer.starts_with(sleep){
        thread::sleep(Duration::from_secs(2));
        ("HTTP/1.1 200 OK", "hello.html")
    }
    else{
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string("404.html").unwrap();

    let res= format!(
        "{}\r\nContent-Length: {} \r\n\r\n{}",
        status_line,
        contents.len(),
        contents
        );

    stream.write(res.as_bytes()).unwrap();
    stream.flush().unwrap();

}
