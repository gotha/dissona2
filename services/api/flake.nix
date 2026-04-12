{
  description = "Disona API Service";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
            "clippy"
            "rustfmt"
            "llvm-tools-preview"
          ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust
            rustToolchain
            lld
            clang

            # Rust tools
            cargo-audit
            cargo-deny
            cargo-outdated
            cargo-watch
            cargo-llvm-cov
            cargo-nextest
            cargo-udeps

            # Build dependencies
            pkg-config
            openssl
            openssl.dev

            # Database client
            postgresql_16

            # Task runner
            just
          ];

          shellHook = ''
            echo "🔧 Disona API Service"
            echo "Run 'just --list' for available commands"
            echo ""

            # Load root .env first (shared secrets like JWT_SECRET)
            if [ -f ../../.env ]; then
              set -a
              source ../../.env
              set +a
            fi

            # Load service-specific .env (overrides root)
            if [ -f .env ]; then
              set -a
              source .env
              set +a
            fi

            export RUST_BACKTRACE=1
            export CARGO_NET_GIT_FETCH_WITH_CLI=true

            # OpenSSL
            export PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkgconfig"
            export OPENSSL_DIR="${pkgs.openssl.dev}"
            export OPENSSL_LIB_DIR="${pkgs.openssl.out}/lib"
            export OPENSSL_INCLUDE_DIR="${pkgs.openssl.dev}/include"
          '';

          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
        };
      }
    );
}
