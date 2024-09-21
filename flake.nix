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
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            go_1_23
            clang
            gtest
            jdk22
            gradle
            nodejs_22
          ];
        };
        formatter = pkgs.nixfmt-rfc-style;
      }
    );
}
