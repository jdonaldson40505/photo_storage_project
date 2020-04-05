use std::io::Read;
use std::net::{TcpListener, TcpStream};

pub(crate) fn connector(){
    let listener = TcpListener::bind("127.0.0.1:5432").unwrap();

    let stream = listener.accept().unwrap().0;
    read_request(stream);
}

fn read_request(mut stream: TcpStream) {
    let mut request_data = String::new();

    stream.read_to_string(&mut request_data);
    println!("{}", request_data);
}