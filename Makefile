.PHONY: bootstrap fmt lint test test-no-default docs contracts ci wasm-demo wasm-demo-serve

bootstrap:
	python3 --version
	rustc --version
	cargo --version

fmt:
	cargo fmt --all

lint:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

test:
	cargo test --workspace --all-features
	python3 scripts/validate_contract_fixtures.py

test-no-default:
	cargo check --workspace --lib --no-default-features

docs:
	cargo doc --workspace --no-deps

contracts:
	python3 scripts/validate_contract_fixtures.py

ci: fmt lint test test-no-default docs

wasm-demo:
	wasm-pack build demo-wasm --target web --release --out-dir www/pkg

wasm-demo-serve:
	cd demo-wasm/www && python3 -m http.server 8080
