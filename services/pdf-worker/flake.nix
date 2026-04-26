{
  description = "Disona PDF Worker";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };

        pythonEnv = pkgs.python312.withPackages (ps: with ps; [
          pip
          virtualenv
        ]);
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Python
            pythonEnv

            # Python tools
            ruff
            mypy

            # PDF processing
            tesseract
            poppler-utils

            # Task runner
            just

            # Required by PyMuPDF binary wheel
            stdenv.cc.cc.lib
          ];

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
            pkgs.stdenv.cc.cc.lib
          ];

          shellHook = ''
            echo "📄 Disona PDF Worker"
            echo "Run 'just --list' for available commands"
            echo ""

            # Load root .env first (shared secrets)
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

            # Create/activate virtual environment
            if [ ! -d ".venv" ]; then
              echo "Creating virtual environment..."
              python -m venv .venv
            fi
            source .venv/bin/activate

            # Install dependencies if needed
            if [ ! -f ".venv/.installed" ]; then
              echo "Installing dependencies..."
              pip install -q -r requirements.txt
              touch .venv/.installed
            fi
          '';
        };
      }
    );
}
