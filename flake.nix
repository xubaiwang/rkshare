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
        toolchainNoLsp = pkgs.rust-bin.stable.latest.default;
        toolchain = toolchainNoLsp.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
          ];
        };
      in
      rec {
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

        packages.pyo3-rkshare =
          with pkgs;
          python3Packages.buildPythonPackage {
            pname = "pyo3-rkshare";
            version = "0.1.0";
            src = ./.;

            pyproject = true;

            build-system = [ rustPlatform.maturinBuildHook ];

            cargoDeps = rustPlatform.importCargoLock {
              lockFile = ./Cargo.lock;
            };

            maturinBuildFlags = [
              "-m"
              "./bindings/python/Cargo.toml"
            ];

            pythonImportsCheck = [ "rkshare" ];

            buildInputs = [
              openssl
            ];

            nativeBuildInputs = [
              pkg-config
              toolchainNoLsp

              rustPlatform.cargoSetupHook
              # rustPlatform.cargoBuildHook
            ];
          };

        devShells.python =
          with pkgs;
          mkShell {
            packages = [
              pyright
              (python3.withPackages (
                ps:
                (with ps; [
                  ipykernel
                  pandas
                ])
                ++ [
                  packages.pyo3-rkshare
                ]
              ))
            ];
            shellHook = ''
              export SHELL=${pkgs.bashInteractive}/bin/bash
            '';
          };
      }
    );
}
