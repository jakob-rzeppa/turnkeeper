# Plugin Runtime Execution

## Make sure to

- `Box::pin(x).await` all recursive execution calls. Recursive async leads to problems, since internally the async tries to create a state-machine containing the sub-calls. A recursive call will lead to a infinitly sized type. This is why we need to add indirection with a `Box::pin(x)`. Indirection means referencing a value through a intermediate e.g. `Box::pin(x)` and makes sure there is no infinitly sized type.
