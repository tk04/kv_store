use crate::command_parser::{parse, CommandType};
use crate::DATA_STORE;
use std::io::{BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};

pub fn listen() {
    let listener = TcpListener::bind("127.0.0.1:6000").unwrap();
    for mut stream in listener.incoming() {
        handler(&mut stream.unwrap());
        // println!("{:?}", stream.unwrap());
    }
}

fn handler(stream: &mut TcpStream) {
    let mut response = String::new();
    stream.write(b"Hello client");
    let mut reader = BufReader::new(stream);
    reader
        .read_to_string(&mut response)
        .expect("could not read");

    let typed = parse(&response);

    match typed.name {
        CommandType::Set => {
            DATA_STORE
                .lock()
                .unwrap()
                .add_key(&typed.values[0], &typed.values[2]);
            return ();
        }
        CommandType::Get => {
            DATA_STORE.lock().unwrap().get_key(&typed.values[0]);
            return ();
        }
    }
}
