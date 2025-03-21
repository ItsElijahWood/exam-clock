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
            <script>
                function updateTime() {
                    const now = new Date();
                    const hours = String(now.getHours()).padStart(2, '0');
                    const minutes = String(now.getMinutes()).padStart(2, '0');
                    document.querySelector('.time').value = `${hours}:${minutes}`;
                }
                setInterval(updateTime, 1000);
                window.onload = updateTime;
            </script>
        </head>
        <body style='margin: 0; background-color: black;'>
            <div style='display: flex; height: 100%; flex-direction: column; gap: 40px; align-items: center; justify-content: center;'>
                <div style='display: flex; gap: 60px; align-items: center;'>
                    <p style='color: white; font-size: 100px;'>Exam Start</p>
                    <input class='start' style='width: 300px; color: red; border: none; background: none; font-size: 100px;' placeholder='--:--'>
                </div>
                <div style='display: flex; gap: 60px; align-items: center;'>
                    <p style='color: white; font-size: 100px;'>Exam Finish</p>
                    <input class='end' style='width: 300px; color: red; border: none; background: none; font-size: 100px;' placeholder='--:--'>
                </div>
                <div style='display: flex; gap: 60px; align-items: center;'>
                    <p style='color: white; font-size: 100px;'>Current Time</p>
                    <input class='time' style='width: 300px; color: red; border: none; background: none; font-size: 100px;' readonly>
                </div>
            </div>
        </body>
        </html>
    ";
    let length = contents.len();

    let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{contents}");

    println!("Go to http://localhost:3000 on google to get to the exam clock.");

    stream.write_all(response.as_bytes()).unwrap();
}
