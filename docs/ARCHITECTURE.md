# Turnkeeper architecture

The Turnkeeper application is split into three parts: the backend, the game master (frontend) and the user (frontend). The game master and users are connecting to the backend via socket.io.

## Basic backend - gm/user socket event sequence

```mermaid
sequenceDiagram
    Gm->>+Backend: "players:update"
    Backend-->>Gm: "players:info"
    Backend-->>-User: "player:info" (only the updated player, if connected)
    Gm->>+Backend: "game:init"
    Backend-->>Gm: "game:info"
    Backend-->>-User: "game:info"
    Gm->>+Backend: "players:create"
    Backend-->>Gm: "players:info"
    Backend-->>User: "player:info" (only the created player, if connected)
    Backend-->>Gm: "game:info" updated Player Order
    Backend-->>-User: "game:info" updated Player Order
```

## Messages event sequence

```mermaid
sequenceDiagram
    Gm->>+Backend: "messages:send"
    Backend-->>Gm: "messages:new"
    Backend-->>-User: "messages:new"
    User->>+Backend: "messages:send"
    Backend-->>Gm: "messages:new"
    Backend-->>-User: "messages:new"
    Gm->>+Backend: "messages:history"
    Backend-->>-Gm: "messages:all"
    User->>+Backend: "messages:history"
    Backend-->>-User: "messages:all"
```

## Gm connection

```mermaid
sequenceDiagram
    actor Gm as Gm Client
    participant Socket as Socket.IO Gm Client
    participant Backend as Backend gmSocket
    participant GmCtrl as GmController

    Gm->>Socket: connect() with auth: { playerName, playerSecret }
    Socket->>Backend: WebSocket connection to /gm namespace with auth: { playerName, playerSecret }

    alt No Backend response

        note over Socket: timeout waiting for server

        Socket->>Gm: Connection failed

    else Backend response

        Backend->>GmCtrl: isConnected()
        GmCtrl-->>Backend: isGmAlreadyConnected

        alt Gm is already connected

            Backend->>Socket: 'connection_error' GM_ALREADY_CONNECTED
            Backend->>Socket: disconnect()
            Socket->>Gm: Connection failed / User needs input credentials again

        else Gm is not already connected

            Backend->>GmCtrl: registerSocket()

            Note over Gm,GmCtrl: Connection established

        end
    end
```

## User connection

```mermaid
sequenceDiagram
    actor User as User Client
    participant Socket as Socket.IO User Client
    participant Backend as Backend userSocket
    participant Auth as Authenticator
    participant PlayerRepo as playerRepository
    participant UserCtrl as UserController

    User->>Socket: connect() with auth: { playerName, playerSecret }
    Socket->>Backend: WebSocket connection to /user namespace with auth: { playerName, playerSecret }

    alt No Backend response

        note over Socket: timeout waiting for server

        Socket->>User: Connection failed / User needs input credentials again

    else Backend response

        Backend->>PlayerRepo: getPlayerIdByName(playerName)
        PlayerRepo-->>Backend: playerName or null

        alt Invalid credentials / Player not found

            Backend->>Socket: 'connection_error', INVALID_CREDENTIALS
            Backend->>Socket: disconnect()
            Socket->>User: Connection failed / User needs input credentials again

        else Valid credentials

            Backend->>Auth: authenticateUser()
            Auth-->>Backend: isAuthenticated

            alt Invalid Secret

                Backend->>Socket: 'connection_error', INVALID_SECRET
                Backend->>Socket: disconnect()
                Socket->>User: Connection failed / User needs input credentials again

            else

                Backend->>UserCtrl: registerSocket()

                Note over User,UserCtrl: Connection established & authenticated

            end
        end
    end
```
