# Disona - Root Orchestration
# Run `just --list` to see all available tasks
#
# Each service has its own justfile with standardized commands:
#   just setup    - Install deps, run migrations
#   just build    - Build the service
#   just check    - Run all quality checks
#   just test     - Run tests with coverage
#   just run      - Run the service locally

set dotenv-load

# Default task
default:
    @just --list

# ============ ALL SERVICES ============

# Setup all services (install deps, run migrations)
setup:
    @echo "Setting up all services..."
    cd services/api && just setup
    cd services/auth && just setup
    cd services/pdf-worker && just setup
    cd services/llm-worker && just setup
    cd services/tts-worker && just setup
    cd frontend && just setup

# Build all services
build:
    @echo "Building all services..."
    cd services/api && just build
    cd services/auth && just build
    cd services/pdf-worker && just build
    cd services/llm-worker && just build
    cd services/tts-worker && just build
    cd frontend && just build

# Run all checks across all services
check:
    @echo "Checking all services..."
    cd services/api && just check
    cd services/auth && just check
    cd services/pdf-worker && just check
    cd services/llm-worker && just check
    cd services/tts-worker && just check
    cd frontend && just check

# Run all tests across all services
test:
    @echo "Testing all services..."
    cd services/api && just test
    cd services/auth && just test
    cd services/pdf-worker && just test
    cd services/llm-worker && just test
    cd services/tts-worker && just test
    cd frontend && just test

# Clean all build artifacts
clean:
    @echo "Cleaning all services..."
    cd services/api && just clean
    cd services/auth && just clean
    cd services/pdf-worker && just clean
    cd services/llm-worker && just clean
    cd services/tts-worker && just clean
    cd frontend && just clean

# ============ INDIVIDUAL SERVICES ============

# Run a specific service's justfile command
svc service *args:
    cd services/{{service}} && just {{args}}

# Run frontend justfile command
fe *args:
    cd frontend && just {{args}}

# ============ INFRASTRUCTURE ============

# Start infrastructure (databases, nats, minio, traefik)
infra-up:
    docker compose up -d postgres-auth postgres-api nats minio traefik

# Stop infrastructure
infra-down:
    docker compose down

# View infrastructure logs
infra-logs *args:
    docker compose logs -f {{args}}

# Reset infrastructure (WARNING: deletes all data)
infra-reset:
    docker compose down -v
    docker compose up -d postgres-auth postgres-api nats minio traefik
    @echo "Waiting for databases to be ready..."
    sleep 5
    just setup

# ============ DOCKER ============

# Build all Docker images
docker-build:
    cd services/api && just docker-build
    cd services/auth && just docker-build
    cd services/pdf-worker && just docker-build
    cd services/llm-worker && just docker-build
    cd services/tts-worker && just docker-build
    cd frontend && just docker-build

# Start all services with Docker Compose
docker-up:
    docker compose up -d

# Stop all Docker services
docker-down:
    docker compose down

# ============ DEVELOPMENT ============

# Start development environment (infra + watch services)
dev:
    @echo "Starting infrastructure..."
    just infra-up
    @echo ""
    @echo "Infrastructure is running. Start services with:"
    @echo "  cd services/api && just watch"
    @echo "  cd services/auth && just watch"
    @echo "  cd frontend && just dev"

# Generate a dev JWT token
dev-token:
    cd services/auth && just dev-token

# Generate a JWT token for a specific user
generate-token user_id email name:
    cd services/auth && just generate-token {{user_id}} {{email}} "{{name}}"

# Verify a JWT token
verify-token token:
    cd services/auth && just verify-token {{token}}
