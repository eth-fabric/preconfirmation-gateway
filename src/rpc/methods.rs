use jsonrpsee::server::RpcModule;

use crate::db::DatabaseContext;

use super::handlers;

pub fn setup_rpc_methods(db_context: DatabaseContext) -> anyhow::Result<RpcModule<DatabaseContext>> {
	let mut module = RpcModule::new(db_context);

	module.register_method("commitmentRequest", handlers::commitment_request_handler)?;
	module.register_method("commitmentResult", handlers::commitment_result_handler)?;
	module.register_method("slots", handlers::slots_handler)?;
	module.register_method("fee", handlers::fee_handler)?;

	Ok(module)
}
