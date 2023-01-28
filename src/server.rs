use crate::command_parser::{parse, CommandType};
use crate::DATA_STORE;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

pub fn listen() {
    let listener = TcpListener::bind("127.0.0.1:6000").unwrap();
    for stream in listener.incoming() {
        handler(&mut stream.unwrap());
    }
}

fn handler(stream: &mut TcpStream) {
    let mut response = String::new();
    stream
        .read_to_string(&mut response)
        .expect("could not read");

    let typed = parse(&response);
    let mut ds = DATA_STORE.lock().unwrap();

    match typed.name {
        CommandType::Set => {
            ds.add_key(&typed.values[0], &typed.values[2]);
            stream.write(b"Value stored\n").expect("err");
            return ();
        }
        CommandType::Get => {
            // let value = ;
            match ds.get_key(&typed.values[0]) {
                Some(val) => {
                    stream
                        .write(format!("got stored: {}\n", val).as_bytes())
                        .expect("err");
                    return ();
                }
                None => {
                    stream.write(b"No value stored\n").expect("err");
                    return ();
                }
            }
        }
    }
}
