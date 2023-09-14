.PHONY: all
all: testgo testrs testts testcs

testgo:
	(cd Go/ && go test -cover ./...)

testrs:
	(cd Rust/ && cargo nextest run)

testts:
	(cd TypeScript/ && bun test)

testcs:
	(cd CSharp/ && dotnet test)
