use eyre::Result;
use std::time::Duration;
use tracing::{error, info};

/// Coordinates the execution of tokio tasks
pub struct TaskCoordinator {
	task_handles: Vec<tokio::task::JoinHandle<Result<()>>>,
}

impl TaskCoordinator {
	/// Create a new task coordinator
	pub fn new() -> Self {
		Self { task_handles: Vec::new() }
	}

	/// Spawn a tokio task
	pub fn spawn_task<F, Fut>(&mut self, name: String, task: F)
	where
		F: FnOnce() -> Fut + Send + 'static,
		Fut: std::future::Future<Output = Result<()>> + Send + 'static,
	{
		let name_clone = name.clone();
		let task_handle = tokio::spawn(async move {
			info!("Starting task: {}", name);
			let result = task().await;
			match &result {
				Ok(_) => info!("Task {} completed successfully", name),
				Err(e) => error!("Task {} failed: {}", name, e),
			}
			result
		});

		self.task_handles.push(task_handle);
		info!("Spawned task: {}", name_clone);
	}

	/// Wait for all tasks to complete
	pub async fn wait_for_all(&mut self) -> Result<()> {
		info!("Waiting for {} tasks to complete", self.task_handles.len());

		for (i, handle) in self.task_handles.drain(..).enumerate() {
			match handle.await {
				Ok(Ok(())) => info!("Task {} completed successfully", i),
				Ok(Err(e)) => error!("Task {} failed: {}", i, e),
				Err(e) => error!("Task {} panicked: {}", i, e),
			}
		}

		Ok(())
	}

	/// Wait for all tasks with timeout
	pub async fn wait_for_all_with_timeout(&mut self, timeout: Duration) -> Result<()> {
		match tokio::time::timeout(timeout, self.wait_for_all()).await {
			Ok(result) => result,
			Err(_) => {
				error!("Task execution timed out after {:?}", timeout);
				Err(eyre::eyre!("Task execution timed out"))
			}
		}
	}

	/// Get the number of spawned tasks
	pub fn task_count(&self) -> usize {
		self.task_handles.len()
	}
}
