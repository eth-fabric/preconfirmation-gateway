use jsonrpsee::core::RpcResult;
use jsonrpsee::Extensions;

pub fn say_hello_handler(_params: jsonrpsee::types::Params<'_>, _context: &(), _extensions: &Extensions) -> RpcResult<String> {
	Ok("lo".to_string())
}