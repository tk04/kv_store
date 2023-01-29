use crate::command_parser::{parse_cmd, Command, CommandType, Response};
use crate::DATA_STORE;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str::FromStr;
use std::thread;

pub fn listen() {
    let listener = TcpListener::bind("127.0.0.1:6000").unwrap();
    for stream in listener.incoming() {
        thread::spawn(|| handler(&mut stream.unwrap()));
    }
}

fn match_cmd(cmd: Command) -> String {
    match cmd.name {
        CommandType::Set => set_res(cmd),
        CommandType::Get => get_res(cmd),
        CommandType::Add => add_res(cmd),
        CommandType::Prepend => prepend_res(cmd),
        CommandType::Append => append_res(cmd),
        CommandType::Delete => delete_res(cmd),
        CommandType::Replace => replace_res(cmd),
        CommandType::FlushAll => flush_all_res(),
    }
}
fn handler(stream: &mut TcpStream) {
    let mut response = String::new();
    stream
        .read_to_string(&mut response)
        .expect("could not read");

    let typed = parse_cmd(&response);
    match typed {
        Ok(val) => {
            stream
                .write(match_cmd(val).as_bytes())
                .expect("socket error");
        }
        Err(_) => {
            stream
                .write(Response::Error.to_string().as_bytes())
                .expect("socket error");
        }
    }
}

fn valid_value(cmd: &Command) -> bool {
    if cmd.values.len() < 3 {
        return false;
    }
    let bytes = usize::from_str(cmd.values[1].as_str());
    match bytes {
        Ok(val) => {
            if cmd.values[2].as_bytes().len() != val {
                return false;
            }
            return true;
        }
        Err(_) => false,
    }
}
fn set_res(cmd: Command) -> String {
    if !valid_value(&cmd) {
        return Response::Error.to_string();
    }

    let mut ds = DATA_STORE.lock().unwrap();
    ds.set_key(&cmd.values[0], &cmd.values[2]);

    return Response::Stored.to_string();
}

fn get_res(cmd: Command) -> String {
    if cmd.values.len() == 0 {
        return Response::Error.to_string();
    }

    let ds = DATA_STORE.lock().unwrap();
    match ds.get_key(&cmd.values[0]) {
        Some(val) => {
            return format!(
                "VALUE {} {} \r\n{val}\r\nEND\r\n",
                cmd.values[0],
                val.as_bytes().len()
            );
        }
        None => Response::NotFound.to_string(),
    }
}
fn append_res(cmd: Command) -> String {
    if !valid_value(&cmd) {
        return Response::Error.to_string();
    }
    let mut ds = DATA_STORE.lock().unwrap();
    if ds.append_key(&cmd.values[0], &cmd.values[2]) {
        return Response::Stored.to_string();
    }
    return Response::NotStored.to_string();
}
fn prepend_res(cmd: Command) -> String {
    if !valid_value(&cmd) {
        return Response::Error.to_string();
    }
    let mut ds = DATA_STORE.lock().unwrap();
    if ds.prepend_key(&cmd.values[0], &cmd.values[2]) {
        return Response::Stored.to_string();
    }
    return Response::NotStored.to_string();
}

fn add_res(cmd: Command) -> String {
    if !valid_value(&cmd) {
        return Response::Error.to_string();
    }
    let mut ds = DATA_STORE.lock().unwrap();

    if !ds.has_key(&cmd.values[0]) {
        ds.set_key(&cmd.values[0], &cmd.values[2]);
        return Response::Stored.to_string();
    }
    return Response::NotStored.to_string();
}
fn delete_res(cmd: Command) -> String {
    if cmd.values.len() == 0 {
        return Response::Error.to_string();
    }
    let mut ds = DATA_STORE.lock().unwrap();
    if ds.delete_key(&cmd.values[0]) {
        return Response::Deleted.to_string();
    }
    return Response::NotFound.to_string();
}
fn replace_res(cmd: Command) -> String {
    if !valid_value(&cmd) {
        return Response::Error.to_string();
    }
    let mut ds = DATA_STORE.lock().unwrap();
    if ds.has_key(&cmd.values[0]) {
        ds.set_key(&cmd.values[0], &cmd.values[2]);
        return Response::Stored.to_string();
    }
    return Response::NotFound.to_string();
}

fn flush_all_res() -> String {
    let mut ds = DATA_STORE.lock().unwrap();
    ds.delete_all();
    return Response::Ok.to_string();
}
