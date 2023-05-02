{
  description = "bookshelf-rs";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/release-22.11";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustVersion = pkgs.rust-bin.stable.latest.default;

        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustVersion;
          rustc = rustVersion;
        };

        rustBuild = rustPlatform.buildRustPackage {
            pname = "bookshelf-rs";
            version = "0.1.0";
            src = ./.;
            nativeBuildInputs = [pkgs.pkg-config ]; # just for the host building the package
            buildInputs = [ pkgs.sqlite ];
            cargoLock.lockFile = ./Cargo.lock;

            PKG_CONFIG_PATH = "${pkgs.sqlite.dev}/lib/pkgconfig";
        };

      in {
        defaultPackage = rustBuild;
        devShell = pkgs.mkShell {
          buildInputs =
            [ (rustVersion.override { extensions = [ "rust-src" ]; }) pkgs.git ];
        };
      }
    );
}