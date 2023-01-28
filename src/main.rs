mod command_parser;
mod data_store;
mod server;
use std::thread;
mod client;

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref VALUES: HashMap<String, String> = HashMap::new();
    static ref DATA_STORE: Mutex<data_store::DataStore> = Mutex::new(data_store::DataStore::new());
}
fn main() {
    thread::spawn(|| client::send());
    server::listen();
    // let mut store = data_store::DataStore::new();
    // // let s = store.get_key("there");
    // store.add_key("new val", "this is a new val");
}
