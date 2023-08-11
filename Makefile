testgo:
	(cd Go/ && go test -cover ./...)

testrs:
	(cd Rust/ && cargo test --lib)

testrs2:
	(cd Rust/ && cargo nextest run)
