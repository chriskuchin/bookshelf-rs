{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  nativeBuildInputs = with pkgs.buildPackages; [
    nodejs_18
    rustc
    cargo
    rustfmt
    rust-analyzer
    clippy
    git
    tig
  ];
}
