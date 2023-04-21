# CHAPP

## Usage

chapp_rs can start a server, that is listening for write and read clients. Write clients can send messages and read clients can read messages sent by writer.

```shell
$ cargo run -- -l <ip-address>:<port>       # start listening for clients
$ cargo run -- -w <ip-address>:<port>       # start a write only client
$ cargo run -- -r <ip-address>:<port>       # start a read only client
```
