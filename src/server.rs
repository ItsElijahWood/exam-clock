use std::{io::Write, net::{TcpListener, TcpStream}};

pub fn server(addr: &str) {
    let addr = format!("127.0.0.1{}", addr);

    println!("Go to http://127.0.0.1:3000 on google to get to the exam clock.");
    let tcp = TcpListener::bind(addr).expect("Error setting up web server: Failed to bind address to TcpListener in file server.rs");

    for stream in tcp.incoming() {
        let mut stream = stream.unwrap();

        index(&mut stream);
    }
}

fn index(mut stream: &TcpStream) {
    let status = "HTTP/1.0 200 OK";
    let contents = "
        <html>
        <head>
            <title>Exam Clock</title>
        </head>
        <body>

        </body>
        </html>
    ";
    let length = contents.len();

    let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{contents}");

    println!("Go to http://localhost:3000 on google to get to the exam clock.");

    stream.write_all(response.as_bytes()).unwrap();
}
