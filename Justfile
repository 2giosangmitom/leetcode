# Build and run C++ tests
cpp:
    #!/usr/bin/env bash
    set -euxo pipefail
    cd C++
    just build
    just test

# Run Go tests
go:
    #!/usr/bin/env bash
    cd Go
    go test -v ./...

# Run Java tests
java:
    #!/usr/bin/env bash
    cd Java
    ./gradlew test

# Run JavaScript tests
javascript:
    #!/usr/bin/env bash
    cd JavaScript
    if [[ ! -d "node_modules" ]]; then
        npm install
    fi
    npm run test