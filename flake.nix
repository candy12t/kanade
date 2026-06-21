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
        inherit (pkgs) lib;
        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };
        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        defaultBuildArgs = {
          pname = cargoToml.package.name;
          version = cargoToml.package.version;
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          doCheck = false;
          meta = {
            description = cargoToml.package.description;
            homepage = cargoToml.package.homepage;
            license = lib.getLicenseFromSpdxId cargoToml.package.license;
            mainProgram = cargoToml.package.name;
            platforms = lib.platforms.darwin;
          };
        };
        buildRustPackage = attrs: rustPlatform.buildRustPackage (defaultBuildArgs // attrs);
      in
      {
        devShells.default = pkgs.mkShell {
          packages = [
            rustToolchain
            pkgs.cargo-dist
          ];
        };

        packages.default = buildRustPackage { };

        packages.ci = buildRustPackage {
          doCheck = true;
          checkPhase = ''
            cargo fmt --all -- --check
            cargo clippy --all-targets --all-features -- -D warnings -A unused
            cargo test
          '';
        };
      }
    );
}
