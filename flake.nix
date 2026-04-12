{
  description = "Disona - Turn documents into intelligent audio";

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
            # Task runner (required for all services)
            just

            # Docker (infrastructure)
            docker-compose

            # Misc utilities
            jq
            curl
            httpie
          ];

          shellHook = ''
            echo "🎧 Disona Development Environment"
            echo ""
            echo "Services have their own devShells. Use:"
            echo "  cd services/api && nix develop"
            echo "  cd services/auth && nix develop"
            echo "  cd services/pdf-worker && nix develop"
            echo "  cd services/llm-worker && nix develop"
            echo "  cd services/tts-worker && nix develop"
            echo "  cd frontend && nix develop"
            echo ""
            echo "Or from root:"
            echo "  just --list    # Show orchestration tasks"
            echo ""
          '';
        };
      }
    );
}
