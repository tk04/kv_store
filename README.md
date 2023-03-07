# Persistent & Memcached compatible Key-Value Store

Backwards compatible key-value store that works with all Memcached clients.

List of supported commands:
- set \<key\> \<flags\> \<expiration\>\r\n\<val\>\r\n
- get \<key\>\r\n
- append \<key\> \<flags\> \<expiration\>\r\n\<val\>\r\n
- prepend \<key\> \<flags\> \<expiration\>\r\n\<val\>\r\n
- add \<key\> \<flags\> \<expiration\>\r\n\<val\>\r\n
- replace \<key\> \<flags\> \<expiration\>\r\n\<val\>\r\n
- flush_all\r\n
- delete \<key\>\r\n

Here is a brief breakdown of what each file in my implementation serves as: 
- `command_parser.rs` : contains all of the command parsing logic
- `data_store.rs`: contains all of the data storage/retrieval logic.
- `server.rs`: handles the execution of commands and concurrency between clients. 
- `main.rs`: Initializes global variables, and starts the server.
- `data_store.txt`: write-ahead log file. Permanent storage of user values/ actions. 
- `client.rs`: Contains client testing code that tests both set & get commands.

## Implementation Details
The main command parsing logic lies in the file `command_parser.rs` , and it first splits the provided query string on the chars \r\n . The first item in the sliced string will hold the main command details (such as command, key, flags, expiration, etc), and the second item (if any), will hold the value. I then separate the first item using spaces as a delimiter. I keep a hash map that keeps track of all supported commands, and check if the provided commands is supported. If it is, I keep parsing the provided string. If not, I send back ERROR\r\n .
Parsing the rest of the query string is straight forward. I defined a struct the has the following fields:
```rust
pub struct Command {
    pub name: CommandType,
    pub values: Vec<String>,
    pub reply: bool,
}
```

The name field holds the command type (set, get, etc). Values holds a string vector that holds the rest of the values (key names, flags, expiration time). The values in the vector are later checked for correctness. The reply field is a boolean, which is instructs us of whether to send a reply back or not.

To keep track of the user fields, I store all values in a text file called data_store.txt . This files serves as a write-ahead log used for data recovery in case the system crashes. The user keys & values are also stored in a hash map for quick data access. When the program starts, it checks if there is a data_store.txt file in the current directory, and if there is, it recovers user operations through parsing the text inside that file, and storing the keys in a global hash map. If the file is not present, we create it.
