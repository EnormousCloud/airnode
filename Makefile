test-airnode-abi:
	cargo test --package airnode-abi -- tests

test-airnode-events:
	cargo test --package airnode-events -- tests

doc-airnode-abi:
	cargo doc --package airnode-abi --open

airnode-log:
	cargo run --package airnode-log --release
