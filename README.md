A simple and naive comparision betwwen diffrent gRPC implementation done during the Software Engineering 20/21 course @ La Sapienza University. 

There are implemntation in Rust, Java and Python of client/server communicatin using gRPC, all sharing the same `.proto` file.

Tested with:
* Java 16 and Maven 3.8
* Rust 1.54
* Python 3.9


## Rust

In the `rust` subfolder:

Run the server with
```bash
cargo run --bin helloworld-server

```
and the client
```bash
cargo run --bin helloworld-client
```

## Java

In the `java/gRPCExampleServer` subfolder:

Run the server with
```bash
mvn compile exec:java -Dexec.mainClass="it.uniroma1.gRPCExampleServer.MyServer"

```
In the `java/gRPCExampleClient` subfolder:
Run the client
```bash
mvn compile exec:java -Dexec.mainClass="it.uniroma1.gRPCExampleClient.MyClient"
```