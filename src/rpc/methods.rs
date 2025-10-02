use jsonrpsee::server::RpcModule;

use crate::types::RpcContext;
use crate::rpc::handlers;

pub fn setup_rpc_methods<T: Send + Sync + 'static>(rpc_context: RpcContext<T>) -> anyhow::Result<RpcModule<RpcContext<T>>> {
	let mut module = RpcModule::new(rpc_context);

	module.register_async_method("commitmentRequest", handlers::commitment_request_handler)?;
	module.register_method("commitmentResult", handlers::commitment_result_handler)?;
	module.register_method("slots", handlers::slots_handler)?;
	module.register_method("fee", handlers::fee_handler)?;

	Ok(module)
}
