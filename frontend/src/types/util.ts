export type DataState<T> =
    | { status: 'uninitialized' }
    | { status: 'loading' }
    | { status: 'error'; error: string }
    | { status: 'success'; data: T };
