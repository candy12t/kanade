{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };
        defaultBuildArgs = {
          pname = "kanade";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          doCheck = false;
        };
        buildRustPackage = attrs: rustPlatform.buildRustPackage (defaultBuildArgs // attrs);
      in
      {
        devShells.default = pkgs.mkShell {
          packages = [ rustToolchain ];
        };

        packages.default = buildRustPackage { };

        packages.ci = buildRustPackage {
          doCheck = true;
          checkPhase = ''
            cargo fmt --all -- --check
            cargo clippy --all-targets --all-features -- -D warnings
            cargo test
          '';
        };
      }
    );
}
