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
    ruby_3_2
    jdk21
    gradle_8
  ];
}
