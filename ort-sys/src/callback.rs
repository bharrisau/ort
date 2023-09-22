use std::{
	ffi::c_void,
	future::Future,
	marker::PhantomData,
	pin::Pin,
	sync::{Arc, Mutex},
	task::{Context, Poll, Waker}
};

pub struct CallbackFuture<T, F> {
	shared_state: Arc<Mutex<SharedState<T>>>,
	_phantom: PhantomData<F>
}

impl<T, F> CallbackFuture<T, F> {
	pub(crate) fn to_onnx(&self) -> (F, *mut c_void) {
		todo!()
	}
}

// Shared state between the future and the waiting thread
struct SharedState<T> {
	result: Option<T>,
	// The waker for the task that `CallbackFuture` is running on.
	waker: Option<Waker>
}

impl<T> SharedState<T> {
	unsafe fn set_output(ptr: *const c_void, output: T) {
		let arc: Arc<Mutex<Self>> = unsafe { Arc::from_raw(ptr as _) };
		let mut shared_state = arc.lock().unwrap();

		assert!(shared_state.result.is_none());
		shared_state.result = Some(output);

		if let Some(wake) = shared_state.waker.take() {
			wake.wake()
		}
	}
}

impl<T, F> Future for CallbackFuture<T, F> {
	type Output = T;
	fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		// Look at the shared state to see if the timer has already completed.
		let mut shared_state = self.shared_state.lock().unwrap();
		if let Some(r) = shared_state.result.take() {
			Poll::Ready(r)
		} else {
			// Set waker so that the thread can wake up the current task
			// when the timer has completed, ensuring that the future is polled
			// again and sees that `completed = true`.
			//
			// It's tempting to do this once rather than repeatedly cloning
			// the waker each time. However, the `TimerFuture` can move between
			// tasks on the executor, which could cause a stale waker pointing
			// to the wrong task, preventing `TimerFuture` from waking up
			// correctly.
			//
			// N.B. it's possible to check for this using the `Waker::will_wake`
			// function, but we omit that here to keep things simple.
			shared_state.waker = Some(cx.waker().clone());
			Poll::Pending
		}
	}
}
