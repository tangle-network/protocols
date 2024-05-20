{
  description = "Webb Protocols and blueprints development environment";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    # Rust
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        lib = pkgs.lib;
        toolchain = pkgs.rust-bin.nightly.latest.default.override
          {
            extensions = [ "rust-src" "clippy" "rustfmt" ];
            targets = [ ];
          };
      in
      {
        devShells.default = pkgs.mkShell {
          name = "protocols";
          nativeBuildInputs = [
            pkgs.pkg-config
            pkgs.clang
            pkgs.libclang.lib
            pkgs.gmp
            # Protocol Buffers
            pkgs.protobuf
            # Mold Linker for faster builds (only on Linux)
            (lib.optionals pkgs.stdenv.isLinux pkgs.om4)
            (lib.optionals pkgs.stdenv.isLinux pkgs.mold)
            (lib.optionals pkgs.stdenv.isDarwin pkgs.darwin.apple_sdk.frameworks.Security)
            (lib.optionals pkgs.stdenv.isDarwin pkgs.darwin.apple_sdk.frameworks.SystemConfiguration)
          ];
          buildInputs = [
            # We want the unwrapped version, wrapped comes with nixpkgs' toolchain
            pkgs.rust-analyzer-unwrapped
            # Finally the toolchain
            toolchain
          ];
          packages = [
            pkgs.cargo-nextest
            pkgs.cargo-machete
            pkgs.cargo-expand
          ];
          # Environment variables
          RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
          LD_LIBRARY_PATH = lib.makeLibraryPath [ pkgs.gmp pkgs.libclang ];
        };
      });
}
