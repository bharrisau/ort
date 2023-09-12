use std::sync::{Arc, RwLock, Weak};

use crate::sys::{OrtEnv, OrtLoggingLevel};
use crate::Api;

pub enum EnvOptions<'a> {
	Singleton,
	Default(LogLevel, &'a str)
}

#[repr(u32)]
pub enum LogLevel {
	#[doc = "< Verbose informational messages (least severe)."]
	Verbose = crate::sys::OrtLoggingLevel_ORT_LOGGING_LEVEL_VERBOSE,
	#[doc = "< Informational messages."]
	Info = crate::sys::OrtLoggingLevel_ORT_LOGGING_LEVEL_INFO,
	#[doc = "< Warning messages."]
	Warning = crate::sys::OrtLoggingLevel_ORT_LOGGING_LEVEL_WARNING,
	#[doc = "< Error messages."]
	Error = crate::sys::OrtLoggingLevel_ORT_LOGGING_LEVEL_ERROR,
	#[doc = "< Fatal error messages (most severe)."]
	Fatal = crate::sys::OrtLoggingLevel_ORT_LOGGING_LEVEL_FATAL
}

static G_ENV: RwLock<Option<Weak<Environment>>> = RwLock::new(Option::None);

#[derive(Debug)]
struct Environment {
	pub(crate) env: *mut OrtEnv
}

unsafe impl Send for Environment {}
unsafe impl Sync for Environment {}

impl Environment {
	pub fn get(options: EnvOptions) -> Arc<Self> {
		let lock = G_ENV.read().unwrap();

		if let Some(arc) = lock.as_ref().and_then(Weak::upgrade) {
			return arc;
		}

		drop(lock);
		let mut lock = G_ENV.write().unwrap();

		if let Some(arc) = lock.as_ref().and_then(Weak::upgrade) {
			return arc;
		}

		let val = Self::create_env(options);
		let ret = Arc::new(val);
		*lock = Some(Arc::downgrade(&ret));
		ret
	}

	fn create_env(options: EnvOptions) -> Self {
		let ptr = match options {
			EnvOptions::Default(level, id) => Api::get().env_create(level as OrtLoggingLevel, id),
			EnvOptions::Singleton => Api::get().env_create(LogLevel::Warning as OrtLoggingLevel, "default")
		};

		Self { env: ptr.unwrap() }
	}
}

impl Drop for Environment {
	fn drop(&mut self) {
		let mut lock = G_ENV.write().unwrap();
		let _ = lock.take();

		Api::get().env_release(self.env);
	}
}
