test_rust:
	(cd ./Rust/ && cargo test --lib )

test_cs:
	(cd ./CSharp/ && dotnet test)

test_go:
	(cd ./Golang/ && go test -cover ./...)

test_ts:
	(cd ./TypeScript/ && npm test)

test_ruby:
	(cd ./Ruby/ && bundle exec rspec -f d)

test_java:
	(cd ./Java/ && gradle test)
