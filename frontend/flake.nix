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

            # Vite reads .env files directly — don't export them
            # to the shell environment (would conflict with Vite's own loading)

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
