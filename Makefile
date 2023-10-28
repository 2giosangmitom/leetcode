test_all: test_rust test_go test_cs test_ts test_ruby

test_rust:
	(cd ./Rust/ && cargo test --lib )

test_cs:
	(cd ./CSharp/ && dotnet test)

test_go:
	(cd ./Golang/ && go test -cover ./...)

test_ts:
	(cd ./TypeScript/ && npm run test)

test_ruby:
	(cd ./Ruby/ && bundle exec rspec -f d)
