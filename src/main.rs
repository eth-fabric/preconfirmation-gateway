use std::net::SocketAddr;

use jsonrpsee::core::client::ClientT;
use jsonrpsee::http_client::HttpClient;
use jsonrpsee::rpc_params;
use jsonrpsee::server::Server;
use tracing_subscriber::util::SubscriberInitExt;

mod db;
mod rpc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let filter = tracing_subscriber::EnvFilter::try_from_default_env()?
		.add_directive("jsonrpsee[method_call{name = \"commitmentRequest\"}]=trace".parse()?);

	tracing_subscriber::FmtSubscriber::builder().with_env_filter(filter).finish().try_init()?;

	// Initialize database connection
	let mut db_client = db::create_connection()?;
	db::test_connection(&mut db_client)?;
	let db_context = db::DatabaseContext::new(db_client);

	let server_addr = run_server(db_context).await?;
	let url = format!("http://{}", server_addr);

	let client = HttpClient::builder().build(url)?;
	let params = rpc_params![1_u64, 2, 3];
	let response: Result<String, _> = client.request("commitmentRequest", params).await;
	tracing::info!("r: {:?}", response);

	Ok(())
}

async fn run_server(db_context: db::DatabaseContext) -> anyhow::Result<SocketAddr> {
	let server = Server::builder().build("127.0.0.1:0".parse::<SocketAddr>()?).await?;
	let module = rpc::setup_rpc_methods(db_context)?;

	let addr = server.local_addr()?;
	let handle = server.start(module);

	// In this example we don't care about doing shutdown so let's it run forever.
	// You may use the `ServerHandle` to shut it down or manage it yourself.
	tokio::spawn(handle.stopped());

	Ok(addr)
}
