testgo:
	(cd Go/ && go test -v -cover ./...)

testrs:
	(cd Rust/ && cargo test --lib)
