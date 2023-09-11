static G_ENV: Mutex<Arc<*const void>> = Mutex::new(Weak::new());

#[derive(Debug, Clone)]
pub struct Environment {
	env: Arc<Mutex<EnvironmentSingleton>>,
	pub(crate) execution_providers: Vec<ExecutionProvider>
}

impl Environment {
	pub fn get() -> Environment {}
}
