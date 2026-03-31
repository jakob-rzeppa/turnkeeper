# Authentication and Authorization

The backend has two auth domains: GM and User.

- GM auth uses `GM_PASSWORD` + GM JWT
- User auth uses username/password + User JWT

## GM Login Flow

```mermaid
sequenceDiagram
    participant Client
    participant Http as POST /gm/login
    participant Handler as GmLoginRequestHandler
    participant JWT as GmJwtGenerator

    Client->>Http: password
    Http->>Handler: login(request)
    Handler->>Handler: compare with GM_PASSWORD
    Handler->>JWT: generate_token()
    JWT-->>Http: token
    Http-->>Client: { token }
```

## User Login/Register

```mermaid
sequenceDiagram
    participant Client
    participant Http as Http Endpoints
    participant Handler as User Request Handler
    participant Repo as UserRepository
    participant JWT as UserJwtGenerator

    Client->>Http: credentials
    Http->>Handler: login/register
    Handler->>Repo: read/write user
    Handler->>JWT: generate_token(user_id)
    Http-->>Client: { token }
```

## Authorization Flow

```mermaid
sequenceDiagram
    participant Client
    participant MW as user_auth_middleware
    participant JWT as UserJwtGenerator
    participant Repo as UserRepository

    Client->>MW: Authorization: Bearer <token>
    MW->>JWT: validate_token()
    MW->>Repo: get_by_id(user_id)
    MW-->>MW: attach User extension
```
