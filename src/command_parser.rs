use super::COMMANDS;
use std::result::Result;
use std::str::Split;

#[derive(Debug)]
pub struct Command {
    pub name: CommandType,
    pub values: Vec<String>,
    // flags: HashMap<Flags, String>,
}
#[derive(Debug, Clone)]
pub enum CommandType {
    Set,
    Get,
    Add,
    Replace,
    Append,
    Prepend,
    Delete,
    FlushAll,
}
pub enum Response {
    Stored,
    NotStored,
    NotFound,
    Error,
    Deleted,
    Ok,
}
impl Response {
    pub fn to_string(&self) -> String {
        match self {
            Response::Stored => "STORED\r\n".to_string(),
            Response::NotStored => "NOT_STORED\r\n".to_string(),
            Response::NotFound => "NOT_FOUND\r\n".to_string(),
            Response::Error => "ERROR\r\n".to_string(),
            Response::Deleted => "DELETED\r\n".to_string(),
            Response::Ok => "OK\r\n".to_string(),
        }
    }
}

fn try_parse(
    cmd_type: CommandType,
    cmd: &mut Split<&str>,
    val: &mut Split<&str>,
) -> Result<Command, String> {
    let mut v: Vec<String> = Vec::new();

    for i in cmd {
        v.push(i.to_string());
    }
    for i in val {
        v.push(i.to_string());
    }
    match cmd_type {
        CommandType::FlushAll => Ok(Command {
            name: cmd_type,
            values: v,
        }),
        _ => {
            if v.len() > 0 {
                return Ok(Command {
                    name: cmd_type,
                    values: v,
                });
            }

            return Err("Parsing Error".to_string());
        }
    }
}

pub fn parse_cmd(cmd: &str) -> Result<Command, String> {
    let mut command = cmd.trim().split("\r\n");
    let mut proto = command.next().unwrap().split(" ");

    match proto.next() {
        Some(val) => match COMMANDS.get(val) {
            Some(t) => try_parse(t.clone(), &mut proto, &mut command),
            None => Err("Parsing Error".to_string()),
        },
        None => Err("Parsing Error".to_string()),
    }
}
