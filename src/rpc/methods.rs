use jsonrpsee::server::RpcModule;

use super::handlers;

pub fn setup_rpc_methods() -> anyhow::Result<RpcModule<()>> {
	let mut module = RpcModule::new(());
	
	module.register_method("say_hello", handlers::say_hello_handler)?;
	
	Ok(module)
}