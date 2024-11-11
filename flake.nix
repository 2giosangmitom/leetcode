{
  description = "LeetCode solutions implemented in C++, Go, Java, JavaScript";
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        format_code = pkgs.writeScriptBin "format_code" ''
          shopt -s globstar

          # Format Java code
          google-java-format -i ./Java/lib/src/**/*.java

          # Format JavaScript code
          deno fmt ./JavaScript

          # Format C++ code
          clang-format -i ./C++/**/*.cpp ./C++/**/*.hpp

          # Format Go code
          cd Go
          go fmt ./...
        '';
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            go
            clang
            gtest
            jdk
            gradle-unwrapped
            tokei
            bashInteractive
            deno
            cmake
            gnumake
            google-java-format
            just
            format_code
          ];
          shellHook = ''
            export SHELL="${pkgs.bashInteractive}/bin/bash"
          '';
        };
        formatter = pkgs.nixfmt-rfc-style;
      }
    );
}
