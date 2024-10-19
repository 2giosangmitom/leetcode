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
      in
      {
        devShells.default = pkgs.mkShellNoCC {
          buildInputs = with pkgs; [
            go
            clang
            gtest
            jdk22
            gradle
            bear
            tokei
            bashInteractive
            deno
            gnumake
          ];
          shellHook = ''
            export GTEST_PKG="${pkgs.gtest}"
            export GTEST_DEV="${pkgs.gtest.dev}"
            export SHELL="${pkgs.bashInteractive}/bin/bash"
          '';
        };
        formatter = pkgs.nixfmt-rfc-style;
      }
    );
}
