testgo:
	(cd Go/ && gotestsum)

testrs:
	(cd Rust/ && cargo nextest run)

testts:
	(cd TypeScript/ && deno test)

benchts:
	(cd TypeScript/ && deno bench)
