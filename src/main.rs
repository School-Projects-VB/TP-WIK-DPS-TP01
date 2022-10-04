use std::net::TcpListener;

fn main() {
    let ping_listen_port = TcpListener::bind("127.0.0.1:8000").unwrap();

    for stream in ping_listen_port.incoming() {
        let _stream = stream.unwrap();

        println!("Connection established!");
    }
}
