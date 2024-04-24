use std::net::TcpListener;

fn main() {
    let port: u16 = 8080;
    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).unwrap();
    println!("[Server] listening on port: {port}");

    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        println!("Yas!!");
    }
}
