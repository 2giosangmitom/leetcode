{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  packages = with pkgs; [
    ruby_3_2
    dotnet-sdk_7
    rustup
    go
    nodejs_21
  ];
}
