testgo:
	(cd Go/ && go test -cover ./...)

testrs:
	(cd Rust/ && cargo nextest run)

testts:
	(cd TypeScript/ && pnpm test)

testcs:
	(cd CSharp/ && dotnet test)
