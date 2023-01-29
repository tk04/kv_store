use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, Seek, SeekFrom, Write},
};

// commmand errors "invalid command", "Invalid key erro", ValueErr (enter key with length of value,
// if the bytes don't match, throw value_err)
pub struct DataStore {
    file: File,
    store: HashMap<String, String>,
}

fn populate_hash(file: &mut File) -> HashMap<String, String> {
    let mut hash: HashMap<String, String> = HashMap::new();
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        let kv = line.unwrap();
        if let Some(idx) = kv.find(" ") {
            if &kv[0..idx] == "delete" {
                // delete key
                hash.remove(&kv[idx + 1..]);
            } else {
                hash.insert(kv[0..idx].to_string(), kv[idx + 1..].to_string());
            }
        }
    }

    return hash;
}
impl DataStore {
    pub fn new() -> Self {
        let mut file = File::options()
            .create(true)
            .read(true)
            .write(true)
            .open("data_store.txt")
            .expect("error while opening data_store.txt");
        let store = populate_hash(&mut file);
        file.set_len(0).expect("error while handling file");

        file.seek(SeekFrom::End(0))
            .expect("error while handling file");

        for (key, value) in store.iter() {
            file.write(format!("{} {}\n", key, value).as_bytes())
                .expect("error while writing to file");
        }
        return Self {
            file: file.try_clone().unwrap(),
            store,
        };
    }
    pub fn has_key(&self, key: &str) -> bool {
        match self.store.get(key) {
            Some(_) => true,
            _ => false,
        }
    }
    pub fn get_key(&self, key: &str) -> Option<&String> {
        return self.store.get(key);
    }
    pub fn set_key(&mut self, key: &str, value: &str) -> bool {
        self.store.insert(key.to_string(), value.to_string());
        self.file
            .write(format!("{} {}\n", key, value).as_bytes())
            .expect("error while writing to file");
        return true;
    }
    pub fn append_key(&mut self, key: &str, value: &str) -> bool {
        match self.store.get(key) {
            Some(val) => {
                let new_val = val.to_string() + &value.to_string();
                self.store.insert(key.to_string(), new_val.clone());
                self.file
                    .write(format!("{} {}\n", key, new_val).as_bytes())
                    .expect("error while writing to file");
                return true;
            }
            None => false,
        }
    }
    pub fn prepend_key(&mut self, key: &str, value: &str) -> bool {
        match self.store.get(key) {
            Some(val) => {
                let new_val = value.to_string() + &val.to_string();
                self.store.insert(key.to_string(), new_val.clone());
                self.file
                    .write(format!("{} {}\n", key, new_val).as_bytes())
                    .expect("error while writing to file");
                return true;
            }
            None => false,
        }
    }
    pub fn delete_key(&mut self, key: &str) -> bool {
        match self.store.get(key) {
            Some(_) => {
                self.store.remove(key);
                self.file
                    .write(format!("delete {}\n", key).as_bytes())
                    .expect("error while writing to file");
                return true;
            }
            None => false,
        }
    }
    pub fn delete_all(&mut self) -> bool {
        self.store.clear();
        self.file.set_len(0).expect("error while handling file");
        self.file
            .seek(SeekFrom::End(0))
            .expect("error while handling file");

        return true;
    }
}
