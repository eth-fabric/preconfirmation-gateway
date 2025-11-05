use common::config::GatewayConfig;
use jsonrpsee::server::RpcModule;

use super::handlers;
use crate::CommitmentsServerState;

pub fn setup_commitment_methods<T: GatewayConfig + Send + Sync + 'static>(
	rpc_context: CommitmentsServerState<T>,
) -> eyre::Result<RpcModule<CommitmentsServerState<T>>> {
	let mut module = RpcModule::new(rpc_context);

	module.register_async_method("commitmentRequest", handlers::commitment_request_handler)?;
	module.register_method("commitmentResult", handlers::commitment_result_handler)?;
	module.register_method("slots", handlers::slots_handler)?;
	module.register_async_method("fee", handlers::fee_handler)?;
	module.register_async_method("generateProxyKey", handlers::generate_proxy_key_handler)?;

	Ok(module)
}
