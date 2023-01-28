use std::net::TcpStream;
#[derive(Debug)]
pub struct Command {
    pub name: CommandType,
    pub values: Vec<String>,
    // flags: HashMap<Flags, String>,
}
#[derive(Debug)]
pub enum CommandType {
    Set, // [key, byte_size, value]
    Get, // stores key
}

fn parse_set(cmd: &String) -> Command {
    let mut splitted = cmd.split("\r\n");
    let v = String::from(splitted.next().expect("error while parsing"));
    let mut vals = v.split(" ");
    vals.next(); // skip set cmd
    return Command {
        name: CommandType::Set,
        values: vec![
            String::from(vals.next().unwrap()),
            String::from(vals.next().unwrap()),
            String::from(splitted.next().unwrap().trim()),
        ],
    };
}

fn parse_get(cmd: &String) -> Command {
    let mut splitted = cmd.split_whitespace();
    splitted.next();
    return Command {
        name: CommandType::Get,
        values: vec![String::from(splitted.next().unwrap().trim())],
    };
}

pub fn parse(cmd: &String) -> Command {
    let mut command = cmd.trim();
    command = &command[0..3];
    if command == "set" {
        return parse_set(&cmd);
    } else if command == "get" {
        return parse_get(&cmd);
    } else {
        panic!("Unsupported command entered: {}", command)
    }
}

impl Command {
    fn send_response(&self, stream: TcpStream) {
        match self.name {
            CommandType::Set => println!("set"),
            CommandType::Get => println!("get"),
        }
    }
}
