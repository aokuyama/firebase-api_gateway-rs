build-App:
	cargo build --release
	cp ./target/release/lambda-container $(ARTIFACTS_DIR)
