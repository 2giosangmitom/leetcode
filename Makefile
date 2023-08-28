testgo:
	(cd Go/ && go test -cover ./...)

testrs:
	(cd Rust/ && cargo nextest run)

testts:
	(cd TypeScript/ && deno test)

testcs:
	(cd CSharp/ && dotnet test)
