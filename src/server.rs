use crate::command_parser::{parse_cmd, Command, CommandType};
use crate::DATA_STORE;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str::FromStr;

pub fn listen() {
    let listener = TcpListener::bind("127.0.0.1:6000").unwrap();
    for stream in listener.incoming() {
        handler(&mut stream.unwrap());
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
            return ();
        }
        Err(_) => {
            stream.write(b"CommandError\r\n").expect("socket error");
            return ();
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
        return "ValueErr\r\n".to_string();
    }

    let mut ds = DATA_STORE.lock().unwrap();
    ds.set_key(&cmd.values[0], &cmd.values[2]);

    return "Stored\r\n".to_string();
}

fn get_res(cmd: Command) -> String {
    if cmd.values.len() < 0 {
        return "KeyErr\r\n".to_string();
    }

    let mut ds = DATA_STORE.lock().unwrap();
    match ds.get_key(&cmd.values[0]) {
        Some(val) => {
            return format!(
                "Value {} {} \r\n{val}\r\nEnd\r\n",
                cmd.values[0],
                val.as_bytes().len()
            );
        }
        None => "KeyErr\r\n".to_string(),
    }
}
fn append_res(cmd: Command) -> String {
    if !valid_value(&cmd) {
        return "ValueErr\r\n".to_string();
    }
    let mut ds = DATA_STORE.lock().unwrap();
    if ds.append_key(&cmd.values[0], &cmd.values[2]) {
        return "STORED\r\n".to_string();
    }
    return "NOT_STORED\r\n".to_string();
}
fn prepend_res(cmd: Command) -> String {
    if !valid_value(&cmd) {
        return "ValueErr\r\n".to_string();
    }
    let mut ds = DATA_STORE.lock().unwrap();
    if ds.append_key(&cmd.values[0], &cmd.values[2]) {
        return "STORED\r\n".to_string();
    }
    return "NOT_STORED\r\n".to_string();
}

fn add_res(cmd: Command) -> String {
    if !valid_value(&cmd) {
        return "ValueErr\r\n".to_string();
    }
    let mut ds = DATA_STORE.lock().unwrap();

    if !ds.has_key(&cmd.values[0]) {
        ds.set_key(&cmd.values[0], &cmd.values[2]);
        return "STORED\r\n".to_string();
    }
    return "NOT_STORED\r\n".to_string();
}
fn delete_res(cmd: Command) -> String {
    if cmd.values.len() < 0 {
        return "KeyErr\r\n".to_string();
    }
    let mut ds = DATA_STORE.lock().unwrap();
    if ds.delete_key(&cmd.values[0]) {
        return "DELETED\r\n".to_string();
    }
    return "NOT_FOUND\r\n".to_string();
}
fn replace_res(cmd: Command) -> String {
    if !valid_value(&cmd) {
        return "ValueErr\r\n".to_string();
    }
    let mut ds = DATA_STORE.lock().unwrap();
    if ds.has_key(&cmd.values[0]) {
        ds.set_key(&cmd.values[0], &cmd.values[2]);
        return "STORED\r\n".to_string();
    }
    return "NOT_FOUND\r\n".to_string();
}
