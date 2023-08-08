npm_i:
	(cd TS/ && npm i)

testgo:
	(cd Go/ && go test -cover)

testts:
	(cd TS/ && npm run test)
