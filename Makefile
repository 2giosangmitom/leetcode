test_all: test_rust test_go test_cs test_js test_dart

test_rust:
	(cd ./Rust/ && cargo test --lib )

test_cs:
	(cd ./CSharp/ && dotnet test)

test_go:
	(cd ./Golang/ && go test -cover ./...)

test_js:
	(cd ./JavaScript/ && npm run test)

test_dart:
	(cd ./Dart/ && dart test)
