# Websocket Session

### WsConnection

Using the game ws connection as an example - it works the same for the other ws connections.

The WsSessionManager is shared between the websocket use cases, since every one needs to get the user and entries in the WsSessionManager do not interfere with each other.

#### Get the ticket

Js Websockets do not support headers. We don't want to send the users jwt in the url, so we use a ticket endpoint. It serves the purpose of authorizing the user and creating a short-lived ticket used to connect to the websocket.

```mermaid
sequenceDiagram
    actor f as Frontend
    participant ws_t as game_ws_ticket (axios HTTP Endpoint)
    participant wsm as WsSessionManager

    f ->> ws_t : HTTP /game/ws/ticket
    note over ws_t, f : Authentication via middleware
    ws_t ->> wsm : pre_connect(user_id)
    wsm ->> wsm : save ticket
    wsm -->> ws_t : ticket
    ws_t -->> f : ticket
```

The ticket and corresponding information (user information) is saved in the WsSessionManager to be used after the connect.

#### Connect

> Blue sections are communications via channels

```mermaid
sequenceDiagram
    actor f as Frontend
    participant ws_c as game_ws_connect (axios WS Endpoint)
    participant wsm as WsSessionManager
    participant gsm as GameSessionManager

    f ->> ws_c : WS /game/ws?ticket=<ticket>
    ws_c ->> wsm : connect(ticket)
    wsm -->> ws_c : user

    ws_c ->> gsm : connect_to_game(user_id)

    alt if not exists
    create participant gs as GameSession
    gsm ->> gs : create
    gs -->> gsm : connector_channel_sender, game_state_channel_receiver_creator
    end
    gsm ->> gsm : get game_state_channel_receiver via game_state_channel_receiver_creator
    gsm -->> ws_c : command_channel_sender, game_state_channel_receiver

    rect rgb(191, 223, 255)
    note over ws_c, gs : now the websocket session can talk to the game session
    ws_c ->> gs : command_channel
    gs ->> ws_c : game_state_channel
    end
```
