{
  description = "LeetCode solutions implemented in Rust, Go, Java, Python3, C, C++ and TypeScript";
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
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            go
            deno
            cargo
            rustc
            rustfmt
            rust-analyzer
            clippy
            zig
            jdk22
            gradle
            python3
          ];
        };
        formatter = pkgs.nixfmt-rfc-style;
      }
    );
}
