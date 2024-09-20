test_go:
	(cd Go && go test ./... -v)

test_javascript:
	(cd JavaScript && npm run test)

test_java:
	(cd Java && ./gradlew cleanTest test)