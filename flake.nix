{
  description = "LeetCode solutions for C++, Rust, Go, TypeScript";
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        packages = with pkgs; [
          rustc
          cargo
          dotnet-sdk_8
          go
          rustfmt
          deno
          cmake
          gcc13
          cargo-nextest
        ];
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = packages;
        };
        formatter = pkgs.nixpkgs-fmt;
      }
    );
}
