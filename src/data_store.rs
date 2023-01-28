use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    os::unix::prelude::FileExt,
};

// commmand errors "invalid command", "Invalid key erro", ValueErr (enter key with length of value,
// if the bytes don't match, throw value_err)
pub struct DataStore {
    file: File,
    store: KV,
}

#[derive(Serialize, Deserialize, Debug)]
struct KV {
    values: HashMap<String, String>,
}

fn populate_hash(file: &mut File) -> KV {
    let mut buff = String::new();

    file.read_to_string(&mut buff)
        .expect("error while reading file");

    let store: KV = serde_json::from_str::<KV>(buff.as_str()).unwrap();

    return store;
}
impl DataStore {
    pub fn new() -> Self {
        let mut file = File::options()
            .create(true)
            .read(true)
            .write(true)
            .open("data_store.json")
            .expect("error while opening data_store.json");
        let store = populate_hash(&mut file);
        return Self {
            file: file.try_clone().unwrap(),
            store,
        };
    }
    pub fn get_key(&self, key: &str) -> Option<&String> {
        return self.store.values.get(key);
    }
    pub fn add_key(&mut self, key: &str, value: &str) -> bool {
        let mut file = File::options()
            .create(true)
            .truncate(true)
            .read(true)
            .write(true)
            .open("data_store.json")
            .expect("error while opening data_store.json");
        // let written = self.file.write_all(format!("{} {}", key, value).as_bytes());
        // println!("bytes: {:?}", bytes);
        // let s = bytes.find(|x| {
        //     println!("char: {:?}", x);
        //
        //     match x {
        //         Ok(v) => {
        //             println!("char: {}", v);
        //             return false;
        //         }
        //         Err(_) => {
        //             println!("ERROR on finding");
        //             return false;
        //         }
        //     }
        // });
        //
        // println!("s found?: {:?}", s);

        self.store.values.insert(key.to_string(), value.to_string());
        serde_json::to_writer(&mut file, &self.store).expect("error while writing data");
        println!("new values: {:?}", self.store);
        return true;
        // match written {
        //     Ok(_) => true,
        //     _ => false,
        // }
    }
}
impl Drop for DataStore {
    fn drop(&mut self) {
        println!("doing some final cleanup lol");
    }
}
