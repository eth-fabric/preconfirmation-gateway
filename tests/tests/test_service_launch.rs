use eyre::Result;
use integration_tests::test_common::TestHarness;

#[tokio::test]
async fn test_commitments_service_launches() -> Result<()> {
	// Build harness with explicit commitments port to trigger service launch
	let harness = TestHarness::builder().with_commitments_port(Some(9876)).build().await?;

	// Check that the service was launched
	assert!(harness.commitments_service.is_some());

	// Verify we can get the URL
	if let Some(service) = &harness.commitments_service {
		assert_eq!(service.url(), "http://127.0.0.1:9876");
		println!("Commitments service launched successfully at {}", service.url());
	}

	Ok(())
}

#[tokio::test]
async fn test_relay_service_launches() -> Result<()> {
	// Build harness with explicit relay port to trigger service launch
	let harness = TestHarness::builder().with_relay_port(Some(9877)).build().await?;

	// Check that the service was launched
	assert!(harness.relay_service.is_some());

	// Verify we can get the URL
	if let Some(service) = &harness.relay_service {
		assert_eq!(service.url(), "http://127.0.0.1:9877");
		println!("Relay service launched successfully at {}", service.url());
	}

	Ok(())
}

#[tokio::test]
async fn test_both_services_launch() -> Result<()> {
	// Build harness with both ports to launch both services
	let harness = TestHarness::builder().with_commitments_port(Some(9878)).with_relay_port(Some(9879)).build().await?;

	// Check that both services were launched
	assert!(harness.commitments_service.is_some());
	assert!(harness.relay_service.is_some());

	if let Some(commitments) = &harness.commitments_service {
		println!("Commitments service: {}", commitments.url());
	}

	if let Some(relay) = &harness.relay_service {
		println!("Relay service: {}", relay.url());
	}

	Ok(())
}

#[tokio::test]
async fn test_no_services_launch_by_default() -> Result<()> {
	// Build harness without specifying ports - services should NOT launch
	let harness = TestHarness::builder().build().await?;

	// Check that NO services were launched
	assert!(harness.commitments_service.is_none());
	assert!(harness.relay_service.is_none());

	println!("No services launched when ports not specified");

	Ok(())
}
