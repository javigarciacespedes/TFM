use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Error al enlazar al puerto");

    println!("Servidor escuchando en http://127.0.0.1:8080...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_request(stream);
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

fn handle_request(mut stream: std::net::TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).expect("Error al leer los datos del stream");

    let request = String::from_utf8_lossy(&buffer[..]);

    let response = match request.lines().next() {
        Some(line) if line.starts_with("GET") => "Respuesta de GET\n",
        Some(line) if line.starts_with("POST") => "Respuesta de POST\n",
        _ => "Método no soportado\n",
    };

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        response.len(),
        response
    );

    stream.write_all(response.as_bytes()).expect("Error al escribir en el stream");
    stream.flush().expect("Error al hacer flush en el stream");
}
