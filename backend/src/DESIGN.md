# Design Considerations

## File Structure

The project is inspired by the clean architecture pattern.

1. `domain`
2. `application`
3. `infrastructure`

### Domain Layer

### Application Layer

#### RequestHandler - Request / Response

Requests and responses are the data transfer objects used by the infrastructure layer (http handler) to communicate with the application layer on stateless requests.

A request is given and a response returned.

```rust
struct LoginRequestHandler {}

impl LoginRequestHandler {
    fn login(request: LoginRequest) -> Result<LoginResponse, Error> {}
}
```

For each Request should exist a separate RequestHandler.

#### CommandHandler - Command / Event

Commands and events are like requests and responses, but for stateful operations while playing.

```rust
struct SomeCommandHandler {}

impl SomeCommandHandler {
    fn some_command(event_emitter: SomeEventEmitterTrait, some_command: SomeCommand) {}
}
```

With command handlers the methods do not return a response, but an event emitter is given, which will take the event and emit it to the clients.

### Infrastructure Layer
