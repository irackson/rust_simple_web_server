use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    // Bind the TCP listener to all network interfaces, port 3000
    let listener = TcpListener::bind("0.0.0.0:3000").unwrap();
    println!("Listening on http://0.0.0.0:3000");

    // Accept incoming connections in a loop
    for stream in listener.incoming() {
        // Unwrap the incoming stream, handle the connection
        let mut stream = match stream {
            Ok(stream) => stream,
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
                continue;
            }
        };

        // Read data from the stream into a buffer
        let mut buffer = [0; 1024];
        let bytes_read = match stream.read(&mut buffer) {
            Ok(bytes) => bytes,
            Err(e) => {
                eprintln!("Failed to read from stream: {}", e);
                continue;
            }
        };

        let request = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Request: {}", request);

        // Determine the response based on the request
        let (status_line, content) = if request.starts_with("GET / ") {
            (
                "HTTP/1.1 200 OK",
                r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Simple Web Server</title>
            </head>
            <body>
                <h1>Hello, World!</h1>
                <p>This is a simple Rust web server.</p>
            </body>
            </html>
            "#,
            )
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404 - Not Found")
        };

        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            content.len(),
            content
        );

        if let Err(e) = stream.write_all(response.as_bytes()) {
            eprintln!("Failed to write to stream: {}", e);
        }

        if let Err(e) = stream.flush() {
            eprintln!("Failed to flush stream: {}", e);
        }
    }
}
