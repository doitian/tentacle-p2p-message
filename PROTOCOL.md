## P2P Message Protocol

Just a simple p2p message protocol.

Node will exchange their reachable peers on connection. The node also notifiers peers about the changes.

### State

* `reachable_peers: PeerId -> Vec<SessionId>`. This is route table used to find the connection to forward a message.
* `pending_message: Option<{ peer_id: PeerId, message: String }>`: Set to the message specified via the command line arguments.

### Messages

#### `peers`

Exchange reachable peers. The same message is also used to notify the changes.

```
{
  "reachable_peers": [PeerId],
  "disconnected_peers": [PeerId]
}
```

#### `message`

Send a message to a peer.

```
{
  "peer_id": PeerId,
  "message": String
}
```

### Transitions

#### On New Connection

Send peer the `peers` message with all the keys in `reachable_peers`.

If there's a `pending_message`, send the message to the peer and set `pending_message` to `None`.

#### On Disconnection

Clear the disconnected sessions from `reachable_peers`. Once there's no session to reach a peer, remove it from the table. If the peer is removed from the table, broadcast `peers` about the disconnected peers.

#### On Receiving `peers` message

For the current peer id and each id in `reachable_peers`, add a record that routes message to this id via current connection session.

For each id in `disconnected_peers`, remove a record that routes message to this id via current connection session.

If there's new ids added to or removed from `reachable_peers`, broadcast the changes via the `peers` message.

#### On Receiving `message` message

If the message target peer id is self, print the message.

Otherwise find if the target peer id exists in the local `reachable_peers`, broadcast it to the found sessions. Otherwise ignore the message.
