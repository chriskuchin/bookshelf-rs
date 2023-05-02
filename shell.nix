{ pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/95dc6a031b8365744241990d0ff51698c1b536d7.tar.gz") {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo
    rustc
    clippy
    rust-analyzer
    rustfmt
    nodejs
    tig
    git
  ];
}
