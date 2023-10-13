serve:
	(cd ./book/ && mdbook serve --open)

test:
	cargo test --lib
