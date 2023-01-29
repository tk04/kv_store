mod command_parser;
use command_parser::CommandType;
mod data_store;
mod server;
// use std::thread;
// use std::time::Duration;
mod client;

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
// use std::thread::sleep;

lazy_static! {
    static ref VALUES: HashMap<String, String> = HashMap::new();
    static ref DATA_STORE: Mutex<data_store::DataStore> = Mutex::new(data_store::DataStore::new());
    static ref COMMANDS: HashMap<&'static str, command_parser::CommandType> = {
        let mut m: HashMap<&'static str, CommandType> = HashMap::new();
        m.insert("get", CommandType::Get);
        m.insert("add", CommandType::Add);
        m.insert("set", CommandType::Set);
        m.insert("append", CommandType::Append);
        m.insert("replace", CommandType::Replace);
        m.insert("prepend", CommandType::Prepend);
        m.insert("delete", CommandType::Delete);
        m.insert("flush_all", CommandType::FlushAll);
        m
    };
}
fn main() {
    // sleep(Duration::from_secs(1));
    // thread::spawn(|| client::send());
    server::listen();
}
