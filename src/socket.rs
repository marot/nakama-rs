use crate::socket_adapter::SocketAdapter;
use std::error::Error;

struct Socket<A: SocketAdapter<E>, E: Error> {
    pub adapter: A,
    _marker: std::marker::PhantomData<E>,
}

impl<A: SocketAdapter<E>, E: Error> Socket<A, E> {
    fn new(adapter: A) -> Self {
        Socket {
            adapter,
            _marker: std::marker::PhantomData,
        }
    }
}
