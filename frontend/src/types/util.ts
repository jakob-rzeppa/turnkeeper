export type DataState<T> =
    | { status: 'loading' }
    | { status: 'error'; error: string }
    | { status: 'success'; data: T };
