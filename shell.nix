let
  pkgs = import <nixpkgs> {};
in

pkgs.mkShell {
  packages = with pkgs; [
    nodejs_20
    dart
    rustup
    dotnet-sdk_7
    go_1_21
  ];
}
