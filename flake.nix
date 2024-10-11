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
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            (final: prev: {
              deno = prev.stdenvNoCC.mkDerivation rec {
                pname = "deno";
                version = "v2.0.0";
                dontBuild = true;
                strictDeps = true;
                nativeBuildInputs = with pkgs; [
                  unzip
                  installShellFiles
                ];
                dontConfigure = true;
                src = prev.fetchurl {
                  url = "https://github.com/denoland/deno/releases/download/${version}/deno-x86_64-unknown-linux-gnu.zip";
                  sha256 = "sha256-0gG4ErvGzCVlAS5SwqnLmWXXaK/Si7wroprmZ79yUKY=";
                };

                unpackPhase = ''
                  runHook preUnpack
                  mkdir $TMPDIR/deno-unpacked
                  unzip $src -d $TMPDIR/deno-unpacked
                  echo "Unpacked contents:"
                  ls -l $TMPDIR/deno-unpacked
                  runHook postUnpack
                '';

                installPhase = ''
                  runHook preInstall
                  install -Dm 755 $TMPDIR/deno-unpacked/deno $out/bin/deno
                  ln -s $out/bin/deno $out/bin/deno-link
                  runHook postInstall
                '';
              };
            })
          ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            go_1_23
            clang
            gtest
            jdk22
            gradle
            bear
            tokei
            bashInteractive
            deno
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
