# Backend Layers

The backend follows a strict inward dependency rule:

- `Infrastructure` depends on `Application`
- `Application` depends on `Domain`
- `Domain` does not depend on outer layers

```mermaid
flowchart LR
    infra[Infrastructure Layer\nHTTP, WS, DB, Auth] --> app[Application Layer\nHandlers, Sessions, Runtime]
    app --> domain[Domain Layer\nEntities, Value Objects, Rules]
```

Layer deep dives:

- [Domain Layer](domain.md)
- [Application Layer](application.md)
- [Infrastructure Layer](infrastructure.md)
