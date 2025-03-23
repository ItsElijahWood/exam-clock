use std::{fs::File, io::{self, Read, Write}, net::{TcpListener, TcpStream}};

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
    let centre_no = get_centre_no().unwrap();

    let contents = format!(r#"
        <html>
        <head>
            <title>Exam Clock</title>
            <script>
                function updateTime() {{
                    const now = new Date();
                    const hours = String(now.getHours()).padStart(2, '0');
                    const minutes = String(now.getMinutes()).padStart(2, '0');
                    const seconds = String(now.getSeconds()).padStart(2, '0');
                    document.querySelector('.time').textContent = `${{hours}}:${{minutes}}:${{seconds}}`;
                }}
                setInterval(updateTime, 1000);
                window.onload = updateTime;
            </script>
        </head>
        <body style='margin: 0; background-color: black;'>
            <div style='display: flex; height: 100%; flex-direction: column; gap: 40px; align-items: center; justify-content: center;'>
                <div style='display: flex; gap: 60px; align-items: center; height: 50px;'>
                    <p style='user-select: none; color: white; font-size: 100px;'>Centre No:</p>
                    <input class='centre-no' value='{}' style='user-select: none; width: 400px; color: red; border: none; background: none; font-size: 100px;' readonly>
                </div>
                <div style='display: flex;'>
                    <div style='display: flex; align-items: center; flex-direction: column;'>
                        <div style='display: flex; gap: 60px; width: 100%; height: 100px;'>
                            <p style='user-select: none; color: white; font-size: 100px;'>Exam Start:</p>
                        </div> 
                        <div style='display: flex; gap: 60px; align-items: center; width: 100%; height: 400px;'>
                            <p style='user-select: none; color: white; font-size: 100px;'>Exam Finish:</p>
                        </div>
                    </div>
                    <div style='gap: 20px; display: flex; flex-direction: column; align-items: center;'>        
                        <input class='start' style='margin-top: 6rem; padding-left: 5rem; width: 400px; color: red; border: none; background: none; font-size: 100px;' placeholder='--:--'>
                        <input class='end' style='user-select: none; padding-left: 5rem; width: 400px; color: red; border: none; background: none; font-size: 100px;' placeholder='--:--'>
                    </div>
                </div>
                <div style='display: flex; gap: 60px; align-items: center; height: 50px;'>
                    <p style='color: white; font-size: 150px;'>Time Now:</p>
                    <div class='time' style='user-select: none; width: 600px; color: red; border: none; background: none; font-size: 150px;' readonly></div>
                </div>
            </div>
        </body>
        </html>
    "#, centre_no);
    let length = contents.len();

    let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{contents}");

    println!("Go to http://localhost:3000 on google to get to the exam clock.");

    stream.write_all(response.as_bytes()).unwrap();
}

fn get_centre_no() -> io::Result<String> {
    let mut file = File::open("centre_no.txt")?;
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Failed to read file in server.rs");

    Ok(contents)
}
