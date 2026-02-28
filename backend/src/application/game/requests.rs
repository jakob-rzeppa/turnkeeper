use uuid::Uuid;

pub struct CreateGameRequest {
    pub name: String,
}

pub struct DeleteGameRequest {
    pub id: Uuid,
}