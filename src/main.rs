use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let address = "127.0.0.1:7878";

    let listener = match TcpListener::bind(address) {
        Ok(listener) => listener,
        Err(error) => {
            eprintln!("Помилка запуску сервера: {}", error);
            return;
        }
    };

    println!("Сервер запущено: http://{}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(error) => {
                eprintln!("Помилка підключення: {}", error);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 4096];

    let bytes_read = match stream.read(&mut buffer) {
        Ok(size) => size,
        Err(error) => {
            eprintln!("Помилка читання: {}", error);
            return;
        }
    };

    if bytes_read == 0 {
        return;
    }

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);

    println!("HTTP Request:\n{}", request);

    let first_line = request.lines().next().unwrap_or("");

    let response = if first_line.starts_with("GET / HTTP/1.1") {
        create_response(
            "200 OK",
            read_file("index.html")
        )
    } else if first_line.starts_with("GET /about HTTP/1.1") {
        create_response(
            "200 OK",
            read_file("about.html")
        )
    } else if first_line.starts_with("POST / HTTP/1.1") {
        create_response(
            "200 OK",
            "<html><body><h1>POST request processed</h1></body></html>".to_string()
        )
    } else {
        create_response(
            "404 Not Found",
            "<html><body><h1>404 Not Found</h1></body></html>".to_string()
        )
    };

    if let Err(error) = stream.write_all(response.as_bytes()) {
        eprintln!("Помилка відповіді: {}", error);
    }
}

fn read_file(file_name: &str) -> String {
    match fs::read_to_string(file_name) {
        Ok(content) => content,
        Err(_) => {
            "<html><body><h1>500 Internal Server Error</h1></body></html>".to_string()
        }
    }
}

fn create_response(status: &str, body: String) -> String {
    format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
        status,
        body.len(),
        body
    )
}
