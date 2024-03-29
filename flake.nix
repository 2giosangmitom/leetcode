{
  description = "LeetCode solutions for Rust, Go, TypeScript and C#";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
        packages = with pkgs; [
          rustup
          dotnet-sdk_8
          go
          deno
          nil
          fish
          statix
          alejandra
          deadnix
          nodejs_20
        ];
      in {
        devShell = pkgs.mkShell {
          buildInputs = packages;
          shellHook = ''
            exec fish
          '';
        };
        formatter = pkgs.alejandra;
      }
    );
}
