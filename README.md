# tentacle-p2p-message

A [tentacle](https://github.com/nervosnetwork/tentacle) example app. See the commits about how to build a tentacle app step by step.

* [Protocol design](PROTOCOL.md)
* See commits to see how to build a tentacle app step by step.

## Example

Start three nodes:

Node A

```
cargo run 3000
```

Start node B and connect to A

```
cargo run 3001 /ip4/127.0.0.1/tcp/3000
```

Remember the peer id in the log, which is last part of the listen address, for example:

```
listen on /ip4/127.0.0.1/tcp/3001/p2p/{{PeerIdB}}
```

Start node C and connect to A

```
cargo run 3002 /ip4/127.0.0.1/tcp/3000
```

C and B are not directly connected, but it is possible to send a message via A:

```
// Connect to C and let C to send a message to B.
// Replace {{PeerIdB}} with the real one in node B's log.
cargo run 3003 /ip4/127.0.0.1/tcp/3002 {{PeerIdB}} 'This is a test message'
```

Check the log of node B:

```
Receive message to self: test
```
