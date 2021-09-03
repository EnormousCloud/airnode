test-airnode-abi:
	cargo test --package airnode-abi -- tests

doc-airnode-abi:
	cargo doc --package airnode-abi && basic-http-server ./target/doc

test-airnode-log:
	cargo run --package airnode-log --release
