# HTTP Request Lifecycle

HTTP routes are declared in `infrastructure/http/mod.rs` and merged in `build_app`.

## Route Groups

- GM routes: `/gm/*`
- User routes: `/user/*`

## Generic Handler Lifecycle

```mermaid
sequenceDiagram
    participant Client
    participant Router as Axum Router
    participant MW as Auth Middleware
    participant Handler as HTTP Handler
    participant App as Application Handler
    participant Repo as Repository

    Client->>Router: HTTP request
    Router->>MW: protected routes only
    MW-->>Router: allow or reject
    Router->>Handler: deserialize + validate input
    Handler->>App: execute use case
    App->>Repo: read/write data
    Repo-->>App: result
    App-->>Handler: response dto or error
    Handler-->>Client: JSON + status code
```

## Protected Route Model

```mermaid
flowchart LR
    req[Incoming request] --> isPublic{Public route?}
    isPublic -- Yes --> handler[Execute handler]
    isPublic -- No --> mw[Run auth middleware]
    mw --> valid{Valid token?}
    valid -- No --> unauthorized[401 response]
    valid -- Yes --> handler
```

## Error Translation

Domain and application errors are converted to HTTP-safe errors by infrastructure error mapping modules.
