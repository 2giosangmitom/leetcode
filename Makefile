testgo:
	(cd Go/ && go test -v -json -cover ./... | gotestfmt)

testrs:
	(cd Rust/ && cargo nextest run)

testts:
	(cd TypeScript/ && deno test)

benchgo:
	(cd Go/ && go test -run=^$ -benchmem -bench . ./...)

benchts:
	(cd TypeScript/ && deno bench)