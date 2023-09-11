use std::sync::{Arc, Mutex, Weak};

use crate::sys::OrtEnv;
use crate::Api;

static G_ENV: Mutex<Option<Weak<Environment>>> = Mutex::new(Option::None);

#[derive(Debug)]
struct Environment {
	env: *mut OrtEnv
}

unsafe impl Send for Environment {}
unsafe impl Sync for Environment {}

impl Environment {
	pub fn get() -> Arc<Self> {
		let mut lock = G_ENV.lock().unwrap();

		if let Some(arc) = lock.as_ref().and_then(|w| w.upgrade()) {
			arc
		} else {
			let ptr = Api::with(|api| api.create_env(0, "info")).unwrap();
			let ret = Arc::new(Environment { env: ptr });
			*lock = Some(Arc::downgrade(&ret));
			ret
		}
	}
}

impl Drop for Environment {
	fn drop(&mut self) {
		let mut lock: std::sync::MutexGuard<'_, Option<Weak<Environment>>> = G_ENV.lock().unwrap();
		let _ = lock.take();

		Api::with(|api| api.release_env(self.env));
	}
}
