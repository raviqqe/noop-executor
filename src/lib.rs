//! No-op executors.

#![no_std]

use core::{
    pin::pin,
    task::{Context, Poll, Waker},
};

/// Blocks on a future.
///
/// It panics if a future is not ready.
pub fn block_on<T>(future: impl Future<Output = T>) -> T {
    let Poll::Ready(result) = pin!(future).poll(&mut Context::from_waker(Waker::noop())) else {
        unreachable!()
    };

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::future::{pending, ready};

    #[test]
    fn resolve_ready_future() {
        assert_eq!(block_on(async { 42 }), 42);
        assert_eq!(block_on(ready(42)), 42);
    }

    #[test]
    #[should_panic]
    fn panic_on_pending_future() {
        assert_eq!(block_on(pending::<usize>()), 42);
    }
}
