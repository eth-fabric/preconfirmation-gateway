use jsonrpsee::server::RpcModule;

use super::handlers;

pub fn setup_rpc_methods() -> anyhow::Result<RpcModule<()>> {
	let mut module = RpcModule::new(());

	module.register_method("commitmentRequest", handlers::commitment_request_handler)?;
	module.register_method("commitmentResult", handlers::commitment_result_handler)?;
	module.register_method("slots", handlers::slots_handler)?;
	module.register_method("fee", handlers::fee_handler)?;

	Ok(module)
}
