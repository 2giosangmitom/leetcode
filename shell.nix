{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  packages = with pkgs; [
    dotnet-sdk_7
    rustup
    go
  ];
}
