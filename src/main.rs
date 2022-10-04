use std::{
	io::{prelude::*, BufReader},
	net::{TcpListener, TcpStream},
};

fn main() {
	let ping_listen_port = "8000";

	for stream in TcpListener::bind(["127.0.0.1", ping_listen_port].join(":")).unwrap().incoming() {
		let _stream = stream.unwrap();

		handle_connection(_stream);
	}
}

fn handle_connection(mut stream: TcpStream) {
	let buf_reader = BufReader::new(&mut stream);
	let http_request: Vec<_> = buf_reader
		.lines()
		.map(|result| result.unwrap())
		.take_while(|line| !line.is_empty())
		.collect();

	let splited = http_request[0].split_whitespace().collect::<Vec<&str>>()[1];
    
	if splited == "/ping" {
		println!("{}", "HTTP GET 202");
	} else {
		println!("Error 404")
	}
}
