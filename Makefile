serve:
	(cd ./book/ && mdbook serve --open)

test_rust:
	(cd ./Rust/ && cargo test --lib )

format_cs:
	dotnet format ./CSharp/CSharp.csproj

test_cs:
	(cd ./CSharp/ && dotnet test)

test_go:
	(cd ./Golang/ && go test -cover ./...)
