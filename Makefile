.PHONY: all
all: testgo testrs testcs

testgo:
	(cd Go/ && go test -cover ./...)

testrs:
	(cd Rust/ && cargo nextest run)

testcs:
	(cd CSharp/ && dotnet test)
