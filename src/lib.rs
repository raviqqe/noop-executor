//! No-op executors.

#![no_std]

use core::{
    pin::pin,
    task::{Context, Poll, Waker},
};

/// Blocks on a future.
pub fn block_on<T>(future: impl Future<Output = T>) -> T {
    let Poll::Ready(result) = pin!(future).poll(&mut Context::from_waker(Waker::noop())) else {
        unreachable!()
    };

    result
}
