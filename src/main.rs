extern crate dotenv;

use std::{fs::File, io::Read, io::Write, net::TcpListener};
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let port: u16 = 8080;
    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).unwrap();
    println!("[Server] listening on port: {port}");

    // Listen the requests
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 1024];

        stream.read(&mut buffer).unwrap();

        // Request the data to ipapi
        let resp = reqwest::blocking::get(format!("https://api.ipapi.is?q={}&key={}", stream.peer_addr().unwrap().ip(), dotenv::var("API_KEY").unwrap())).unwrap().text().unwrap();

        let mut file = File::options().append(true).open("data.json").unwrap();
        let _ = writeln!(&mut file, "{}", resp);
        let _ = file.sync_all();
    }
}
