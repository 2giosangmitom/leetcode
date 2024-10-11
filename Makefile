all: test_go test_javascript test_java test_cpp

test_go:
	(cd Go && go test ./... -v)

test_javascript:
	(cd JavaScript && deno test)

test_java:
	(cd Java && ./gradlew cleanTest test)

test_cpp:
	(cd C++ && make test)