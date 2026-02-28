# Turnkeeper - Architecture Documentation

## Table of Contents

- [System Overview](#system-overview)
- [Architecture Principles](#architecture-principles)
- [Layer Architecture](#layer-architecture)
- [Domain Model](#domain-model)
- [Authentication & Authorization](#authentication--authorization)
- [Data Flow](#data-flow)
- [API Design](#api-design)
- [WebSocket Communication](#websocket-communication)

## System Overview

Turnkeeper is a turn-based game management system that allows Game Masters (GMs) to create and manage games while multiple users can connect and participate. The system follows clean architecture principles with a clear separation between domain, application, and infrastructure layers.

## Architecture Principles

### Clean Architecture

The backend follows clean architecture principles with three distinct layers:

1. **Domain Layer** - Pure business logic, no external dependencies, except Uuid
2. **Application Layer** - Use cases and orchestration via Request/Event Handlers
3. **Infrastructure Layer** - External concerns (HTTP, WebSockets, Database, Auth)

### Dependency Rule

Dependencies flow inward: Infrastructure → Application → Domain. The domain layer has no dependencies on outer layers.

```mermaid
graph LR
    INFRA[Infrastructure<br/>HTTP, WS, DB, Auth] -->|depends on| APP[Application<br/>Handlers, DTOs]
    APP -->|depends on| DOM[Domain<br/>Entities, Value Objects]

    style DOM fill:#e8f5e9
    style APP fill:#fff3e0
    style INFRA fill:#fce4ec
```

## Layer Architecture

### Domain Layer

The domain layer contains pure business logic with entities, value objects, and domain events.

```mermaid
classDiagram
    class Game {
        -Uuid id
        -String name
        -Vec~Player~ players
        -u32 round_number
        -usize current_player_index
        +new(id, name) Game
        +add_player(player) Result
        +id() Uuid
    }

    class Player {
        -Uuid id
        -Option~User~ user
        -Vec~Stat~ stats
        +new(id) Player
        +add_user(user) void
        +try_add_stat(stat) Result
        +try_add_string_stat() Result
        +try_add_number_stat() Result
        +try_add_bool_stat() Result
        +name() String
    }

    class User {
        -Uuid id
        -UserName name
        -UserPassword password
        +try_new() Result
        +name() String
        +password() String
    }

    class Stat {
        -Uuid id
        -StatKey key
        -StatKind kind
        +try_new_string_stat() Result
        +try_new_number_stat() Result
        +try_new_bool_stat() Result
    }

    class StatKey {
        -String value
        +try_new(value) Result
    }

    class StatKind {
        <<enum>>
        Number(NumberStatValue)
        String(StringStatValue)
        Boolean(BooleanStatValue)
    }

    class UserName {
        -String value
        +try_new(value) Result
    }

    class UserPassword {
        -String value
        +try_new(value) Result
    }

    Game "1" *-- "0..*" Player
    Player "1" *-- "0..1" User
    Player "1" *-- "0..*" Stat
    Stat "1" *-- "1" StatKey
    Stat "1" *-- "1" StatKind
    User "1" *-- "1" UserName
    User "1" *-- "1" UserPassword
```

### Application Layer

The application layer implements use cases through Request Handlers (for HTTP requests) and Event Handlers (for WebSocket events).

```mermaid
graph TB
    subgraph "Application Layer"
        subgraph "Request Handlers"
            ULogin[UserLoginRequestHandler]
            URegister[UserRegisterRequestHandler]
            UAuth[UserAuthenticateRequestHandler]
            GLogin[GmLoginRequestHandler]
            GameCreate[CreateGameRequestHandler]
            GameDelete[DeleteGameRequestHandler]
            GameOverview[OverviewGamesRequestHandler]
        end

        subgraph "Game Session"
            GameSess[GameSession]
            GmConnState[GmConnectionState<br/>None / Pending / Connected]
            UserConnState[UserConnectionState<br/>None / Pending / Connected<br/>Per User]
        end

        subgraph "Contracts/Interfaces"
            UserRepo[UserRepositoryContract]
            GameRepo[GameRepositoryContract]
            GmConnContract[GmConnectionContract]
            JwtGen[JwtGeneratorContract]
            JwtVal[JwtValidatorContract]
        end
    end

    ULogin --> UserRepo
    ULogin --> JwtGen
    URegister --> UserRepo
    URegister --> JwtGen
    UAuth --> UserRepo
    UAuth --> JwtVal
    GameCreate --> GameRepo
    GameDelete --> GameRepo
    GameOverview --> GameRepo
    GameSess --> GameRepo
    GameSess --> GmConnContract
    GameSess --> GmConnState
    GameSess --> UserConnState

    style ULogin fill:#e1f5ff
    style URegister fill:#e1f5ff
    style UAuth fill:#e1f5ff
    style GLogin fill:#fff3e0
    style GameCreate fill:#f1f8e9
    style GameDelete fill:#f1f8e9
    style GameOverview fill:#f1f8e9
    style GameSess fill:#ffe0b2
    style GmConnState fill:#ffe0b2
    style UserConnState fill:#ffe0b2
```

### Infrastructure Layer

The infrastructure layer handles external concerns.

```mermaid
graph TB
    subgraph "Infrastructure Layer"
        subgraph "HTTP"
            Routes[Route Definitions]
            UserHttp[User HTTP Handlers]
            GmHttp[GM HTTP Handlers]
            GameHttp[Game HTTP Handlers]
        end

        subgraph "WebSocket"
            WsHandler[WebSocket Handlers<br/>GM + User]
            SessionMgr[GameSessionManager]
            GmConn[WebSocketGmConnection]
            UserConn[WebSocketUserConnection]
        end

        subgraph "Authentication"
            AuthMgr[AuthManager]
            UserJWT[UserJwtGenerator/Validator]
            GmJWT[GmJwtGenerator/Validator]
        end

        subgraph "Persistence"
            RepoMgr[RepositoryManager]
            UserRepo[SqliteUserRepository]
            GameRepo[SqliteGameRepository]
            DB[(SQLite Pool)]
        end
    end

    Routes --> UserHttp
    Routes --> GmHttp
    Routes --> GameHttp

    WsHandler --> SessionMgr
    SessionMgr --> GmConn
    SessionMgr --> UserConn

    AuthMgr --> UserJWT
    AuthMgr --> GmJWT

    RepoMgr --> UserRepo
    RepoMgr --> GameRepo

    UserRepo --> DB
    GameRepo --> DB

    style Routes fill:#fff3e0
    style AuthMgr fill:#e1f5ff
    style RepoMgr fill:#f1f8e9
    style SessionMgr fill:#ffe0b2
    style DB fill:#f3e5f5
```

## Domain Model

### Entity Relationships

```mermaid
erDiagram
    GAME ||--o{ PLAYER : contains
    PLAYER ||--o| USER : references
    PLAYER ||--o{ STAT : has

    GAME {
        uuid id PK
        string name
        u32 round_number
        usize current_player_index
    }

    PLAYER {
        uuid id PK
        uuid user_id FK
    }

    USER {
        uuid id PK
        string name
        string password
    }

    STAT {
        uuid id PK
        uuid player_id FK
        string key
        variant value
    }
```

### Aggregate Boundaries

```mermaid
graph TB
    subgraph "Game Aggregate"
        Game[Game Entity<br/>Aggregate Root]
        Players[Player Entities]
        Stats[Stat Entities]
        Game --> Players
        Players --> Stats
    end

    subgraph "User Aggregate"
        User[User Entity<br/>Aggregate Root]
    end

    Players -.->|References| User

    style Game fill:#e8f5e9,stroke:#4caf50,stroke-width:3px
    style User fill:#e8f5e9,stroke:#4caf50,stroke-width:3px
```

## Client Connection

Client connection is handled in two ways:

1. REST Api - RequestHandlers
2. Websockets - GameSession

```mermaid
graph TB
    subgraph "GM Device"
        GM[GM Client<br/>Vue.js SPA]
    end

    subgraph "Backend Server"
        API[RequestHandlers]
        WS[GameSession]
    end

    subgraph "User Devices"
        U1[User 1 Client<br/>Vue.js SPA]
        U2[User 2 Client<br/>Vue.js SPA]
        U3[User N Client<br/>Vue.js SPA]
    end

    GM <-->|HTTP| API
    GM <-->|WS| WS
    U1 <-->|HTTP| API
    U1 <-->|WS| WS
    U2 <-->|HTTP| API
    U2 <-->|WS| WS
    U3 <-->|HTTP| API
    U3 <-->|WS| WS

    style GM fill:#e1f5ff
    style API fill:#fff3e0
    style WS fill:#fff3e0
    style U1 fill:#f3e5f5
    style U2 fill:#f3e5f5
    style U3 fill:#f3e5f5
```

## Authentication & Authorization

### Authentication Flow - GM

```mermaid
sequenceDiagram
    participant GM as GM Client
    participant API as HTTP API
    participant Handler as GmLoginHandler
    participant JWT as GmJwtGeneratorContract

    GM->>+API: POST /gm/login<br/>{password}
    API->>+Handler: GmLoginRequest
    Handler->>Handler: Validate password<br/>against ENV variable
    Handler->>+JWT: generate_token()
    JWT-->>-Handler: JWT Token
    Handler-->>-API: GmLoginResponse
    API-->>-GM: 200 OK<br/>{token}

    Note over GM: Store token in localStorage
```

### Authentication Flow - User

```mermaid
sequenceDiagram
    participant User as User Client
    participant API as HTTP API
    participant LoginH as UserLoginHandler
    participant RegisterH as UserRegisterHandler
    participant JWT as UserJwtGeneratorContract
    participant Repo as UserRepositoryContract

    alt User Registration
        User->>+API: POST /user/register<br/>{name, password}
        API->>+RegisterH: UserRegisterRequest
        RegisterH->>+Repo: create_user()
        Repo-->>-RegisterH: User entity
        RegisterH->>+JWT: generate_token(user_id)
        JWT-->>-RegisterH: JWT Token
        RegisterH-->>-API: UserTokenResponse
        API-->>-User: 201 Created<br/>{token}
    else User Login
        User->>+API: POST /user/login<br/>{name, password}
        API->>+LoginH: UserLoginRequest
        LoginH->>+Repo: find_by_name(name)
        Repo-->>-LoginH: User entity
        LoginH->>LoginH: Validate password
        LoginH->>+JWT: generate_token(user_id)
        JWT-->>-LoginH: JWT Token
        LoginH-->>-API: UserTokenResponse
        API-->>-User: 200 OK<br/>{token}
    end

    Note over User: Store token in cookies
```

### Authorization Middleware

```mermaid
graph LR
    Request[HTTP Request] --> CheckToken{Token<br/>Present?}
    CheckToken -->|No| Reject[401 Unauthorized]
    CheckToken -->|Yes| Validate{Valid<br/>Token?}
    Validate -->|No| Reject
    Validate -->|Yes| CheckRole{Check<br/>Role}
    CheckRole -->|GM Required| GMCheck{Is GM<br/>Token?}
    CheckRole -->|User| UserCheck{Is User<br/>Token?}
    GMCheck -->|Yes| Allow[Process Request]
    GMCheck -->|No| Reject403[403 Forbidden]
    UserCheck -->|Yes| Allow
    UserCheck -->|No| Reject403

    style Allow fill:#c8e6c9
    style Reject fill:#ffcdd2
    style Reject403 fill:#ffcdd2
```

## Data Flow

### HTTP Request Flow

```mermaid
sequenceDiagram
    participant Client
    participant Router as Axum Router
    participant HTTP as HTTP Handler
    participant Handler as Request Handler
    participant Repo as Repository

    Client->>+Router: HTTP Request
    Router->>+HTTP: Extract & Validate
    HTTP->>+Handler: Call with Request DTO
    Handler->>+Repo: Repository Call
    Repo-->>-Handler: Domain Entity
    Handler->>Handler: Business Logic
    Handler-->>-HTTP: Response DTO
    HTTP-->>-Router: HTTP Response
    Router-->>-Client: JSON Response
```

### WebSocket Connection & Event Flow

```mermaid
sequenceDiagram
    participant Client
    participant WS as WebSocket Handler
    participant SessionMgr as GameSessionManager
    participant Session as GameSession
    participant Game as Game Aggregate

    Client->>+WS: POST /gm/ws/ticket/{game_id}
    WS->>+SessionMgr: get_or_create_session(game_id)
    SessionMgr-->>-WS: GameSession
    WS->>+Session: gm_pre_connect()
    note over Session: GmConnectionState: None → Pending
    Session-->>-WS: ticket
    WS-->>-Client: { url: "ws://.../gm/ws/{id}?ticket=..." }

    Client->>+WS: GET /gm/ws/{id}?ticket=...
    WS->>+Session: gm_connect(ticket, connection)
    note over Session: Validate ticket, GmConnectionState: Pending → Connected
    Session->>Session: broadcast_game_state()
    Session-->>Client: Full GmGameInfo (initial state)

    loop Event Loop
        Client->>Session: JSON GameEvent
        Session->>+Game: handle_event(event)
        Game->>Game: Update State
        Game-->>-Session: Result
        Session->>Session: broadcast_game_state()
        note over Session: Broadcasts to GM + all connected users
        Session-->>Client: Full GmGameInfo
    end

    Client->>Session: Close
    note over Session: GmConnectionState: Connected → None
    Session-->>-WS: Ok
    deactivate WS

    Note over Session,Game: Event logging (games_log) is defined<br/>but not yet active (commented out)
```

### User WebSocket Connection Flow

```mermaid
sequenceDiagram
    participant User
    participant WS as WebSocket Handler
    participant SessionMgr as GameSessionManager
    participant Session as GameSession
    participant Game as Game Aggregate

    User->>+WS: POST /user/ws/ticket/{game_id}
    WS->>+SessionMgr: get_or_create_session(game_id)
    SessionMgr-->>-WS: GameSession
    WS->>+Session: user_pre_connect(user)
    note over Session: UserConnectionState: None → Pending
    Session-->>-WS: ticket
    WS-->>-User: { url: "ws://.../user/ws/{id}?ticket=...&user_id=..." }

    User->>+WS: GET /user/ws/{id}?ticket=...&user_id=...
    WS->>+Session: user_connect(user_id, ticket, connection)
    note over Session: Validate ticket, UserConnectionState: Pending → Connected
    Session->>Session: broadcast_game_state()
    Session-->>User: Full GmGameInfo (initial state)

    loop Event Loop
        User->>Session: JSON GameEvent
        Session->>+Game: handle_event(event)
        Game->>Game: Update State
        Game-->>-Session: Result
        Session->>Session: broadcast_game_state()
        note over Session: Broadcasts to GM + all connected users
        Session-->>User: Full GmGameInfo
    end

    User->>Session: Close
    note over Session: UserConnectionState: Connected → None
    Session-->>-WS: Ok
    deactivate WS

    Note over Session: Multiple users can connect<br/>to the same GameSession simultaneously
```

## Error Handling

### Error Propagation

```mermaid
graph BT
    DomainErr[Domain Errors<br/>UserError, GameError, GmError] -->|From| InfraErr[Infrastructure Errors<br/>HttpError]
    InfraErr -->|IntoResponse| Response[HTTP Response<br/>Status Code + JSON]

    DomainErr -.->|Contains| Kind[ErrorKind Enum]

    style DomainErr fill:#e8f5e9
    style InfraErr fill:#fce4ec
    style Response fill:#e1f5ff
```

## Key Design Decisions

### 1. UUID as Primary Keys

UUIDs are used for all entity IDs because:

- Allows easy generation
- No need for database round-trips to get IDs

### 2. Request/Response vs Event

- **HTTP**: Request → Handler → Response (stateless)
- **WebSocket**: GameEvent → Handler → Apply to Game → Full State Response

### 3. Current WebSocket Events

The `GameEvent` enum currently supports:

- `AddPlayer` — adds an anonymous player
- `ChangePlayerOrder(Vec<String>)` — reorders players by UUID
- `Debug(String)` — prints a debug message to the server console

### 4. Unimplemented / Partial Features

The following are defined in code but not yet fully functional:

- `SqliteGameRepository::delete()` — will panic (`todo!()`)
- `SqliteGameRepository::log_event()` / `get_game_history()` — event sourcing persistence (`todo!()`)
- Event logging in `GameSession::handle_event()` — the `log_event()` call is commented out
- `GameEvent::is_user_permitted()` — defined but never enforced; user clients can send all events
- User frontend game view — WebSocket connection is established but no `GamePage` component exists to display game state after connecting
