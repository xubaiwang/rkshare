{
  description = "Rust dev shell template";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./toolchain.toml;
      in
      {
        devShells.default =
          with pkgs;
          mkShell {
            packages = [
              toolchain
              maturin
              pyright
              cargo-bloat
            ];
            buildInputs = [
              openssl
              python3
            ];
            nativeBuildInputs = [
              pkg-config
            ];
            shellHook = ''
              alias rkshare="cargo run --quiet --"
              export SHELL=${pkgs.bashInteractive}/bin/bash
            '';
          };
      }
    );
}
