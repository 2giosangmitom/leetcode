testgo:
	(cd Go/ && go test -v -json -cover ./... | gotestfmt)

testrs:
	(cd Rust/ && cargo nextest run)

testts:
	(cd TypeScript/ && deno test)

benchts:
	(cd TypeScript/ && deno bench)