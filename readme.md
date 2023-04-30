# Orbit

Orbit is a message store to:

  1. Store a value to Redis for a given key, we say it is a channel name or a room name.
  2. Accept connection to subscribe to value change
  3. Appoint a connection to be the host of the value
  4. Only the host can update the value
  5. Everyone can emit action to the value,
  6. Propagate incoming actions to the host and the host will reply to orbit the updated value
  7. Broadcast updated value
  8. If the current host disconnected, select a new host
  9. (Day 2) If all connections get disconnected, destroy the value in Redis (Unless explicitly say keep it)

It is aiming to share states between boardgame sessions, which one of the sessions being the host to do all the calculations so that my server can keep thin and stupid, and the boardgame repo doesn't need to care about networking logic.

## Development

Start the development server

```bash
cargo run
```

Run the tests

```bash
cargo test
```
