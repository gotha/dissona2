{
  description = "Disona Frontend";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Node.js (npm is bundled)
            nodejs_20

            # Task runner
            just
          ];

          shellHook = ''
            echo "🎨 Disona Frontend"
            echo "Run 'just --list' for available commands"
            echo ""

            # Load root .env first (shared secrets)
            if [ -f ../.env ]; then
              set -a
              source ../.env
              set +a
            fi

            # Load service-specific .env (overrides root)
            if [ -f .env ]; then
              set -a
              source .env
              set +a
            fi

            # Install dependencies if needed
            if [ ! -d "node_modules" ]; then
              echo "Installing dependencies..."
              npm ci
            fi
          '';
        };
      }
    );
}
