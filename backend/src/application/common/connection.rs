pub enum ConnectionMessage<T> {
    Message(T),
    Close,
}

/// The ConnectionContract defines the interface for managing persistent connections in the application layer.
///
/// It should be implemented as a wrapper around the actual connection management logic, allowing for abstraction and separation of concerns between the application layer and the underlying infrastructure.
pub trait ConnectionContract<R, S> {
    /// Receives the next message from the connection.
    fn recv(&self) -> impl Future<Output = ConnectionMessage<R>> + Send;

    /// Sends a message to the connected client.
    fn send(&self, msg: S) -> impl Future<Output = ()> + Send;
}
