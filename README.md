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
