test_all: test_rust test_go test_cs test_js

format:
	dotnet format ./CSharp/CSharp.csproj && (cd Rust && cargo fmt) && (cd Golang && go fmt ./...)

test_rust:
	(cd ./Rust/ && cargo test --lib )

test_cs:
	(cd ./CSharp/ && dotnet test)

test_go:
	(cd ./Golang/ && go test -cover ./...)

test_js:
	(cd ./JavaScript/ && npm run test)

serve:
	(cd ./book/ && mdbook serve --open)
