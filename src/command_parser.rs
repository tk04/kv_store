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
    // FlushAll,
}
pub enum Error {
    CmdErr,
    ValueErr,
    KeyErr,
}

fn try_parse(
    cmd_type: CommandType,
    cmd: &mut Split<&str>,
    val: &mut Split<&str>,
) -> Result<Command, Error> {
    let mut v: Vec<String> = Vec::new();

    for i in cmd {
        v.push(i.to_string());
    }
    for i in val {
        v.push(i.to_string());
    }

    if v.len() > 0 {
        return Ok(Command {
            name: cmd_type,
            values: v,
        });
    }

    return Err(Error::CmdErr);
}

pub fn parse_cmd(cmd: &str) -> Result<Command, Error> {
    let mut command = cmd.trim().split("\r\n");
    let mut proto = command.next().unwrap().split(" ");

    match proto.next() {
        Some(val) => match COMMANDS.get(val) {
            Some(t) => try_parse(t.clone(), &mut proto, &mut command),
            None => Err(Error::CmdErr),
        },
        None => Err(Error::CmdErr),
    }
}
