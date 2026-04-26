# dissona2

This project is set up with the [BMAD Method](https://docs.bmad-method.org/) (Build More Architect Dreams) - an AI-driven agile development framework.

## Getting Started

### Prerequisites

- [Nix](https://nixos.org/) with flakes enabled
- Docker & Docker Compose
- An AI-powered IDE: Cursor, Claude Code, or Augment Code CLI (auggie)

### Running the Project

1. **Start infrastructure** (PostgreSQL, NATS, MinIO):

```bash
docker compose up -d
```

2. **Start all services** (auth, api, pdf-worker, frontend):

```bash
just run
```

This launches [process-compose](https://f1bonacc1.github.io/process-compose/) with a TUI showing all services.

| Service | Port |
|---------|------|
| Frontend | http://localhost:15003 |
| API | http://localhost:15002 |
| Auth | http://localhost:15001 |
| PostgreSQL (auth) | 15010 |
| PostgreSQL (api) | 15011 |
| NATS | 15020 |
| MinIO | 15030 |

### Using BMAD

1. **Start with help**: In your AI IDE, invoke the `bmad-help` skill to understand where you are and what to do next.

2. **Key workflows**:
   - `bmad-brainstorming` - Ideation and exploration
   - `bmad-create-prd` - Create Product Requirements Document
   - `bmad-create-architecture` - Design technical architecture
   - `bmad-create-epics-and-stories` - Break down work into epics/stories
   - `bmad-dev-story` - Implement a story

3. **Always use fresh chats** for each workflow.

## IDE Configuration

### Cursor

Skills are installed in `.cursor/skills/`. Use skills by name:
```
bmad-help
bmad-create-prd
bmad-agent-architect
```

### Augment Code CLI (auggie)

Skills are symlinked to `.augment/skills/` and `.agents/skills/`. Use the same skill names:
```bash
auggie
# Then invoke skills: bmad-help, bmad-create-prd, etc.
```

Or use the `/skills` command to see all available skills.

## Project Structure

```
.
├── _bmad/                          # BMAD configuration and modules
│   ├── _config/                    # Core configuration
│   ├── core/                       # Core agents, tasks, tools
│   └── bmm/                        # BMad Method module
├── _bmad-output/                   # Generated artifacts
│   ├── planning-artifacts/         # PRD, architecture, epics
│   └── implementation-artifacts/   # Sprint status, etc.
├── docs/                           # Project documentation
├── .cursor/skills/                 # Cursor IDE skills
├── .augment/skills/                # Augment Code skills (symlinks)
└── .agents/skills/                 # Agent skills (symlinks)
```

## Available Skills (43 total)

### Agents
- `bmad-agent-analyst` - Analysis and research
- `bmad-agent-architect` - Technical architecture
- `bmad-agent-dev` - Development
- `bmad-agent-pm` - Product management
- `bmad-agent-qa` - Quality assurance
- `bmad-agent-ux-designer` - UX design
- `bmad-agent-tech-writer` - Documentation

### Workflows
- `bmad-help` - Get guidance on what to do next
- `bmad-brainstorming` - Ideation
- `bmad-create-prd` - Product Requirements
- `bmad-create-architecture` - Architecture design
- `bmad-create-epics-and-stories` - Epic/story breakdown
- `bmad-sprint-planning` - Sprint initialization
- `bmad-dev-story` - Story implementation
- `bmad-code-review` - Code review

### Tools
- `bmad-quick-dev` - Quick development flow
- `bmad-check-implementation-readiness` - Validate planning
- `bmad-correct-course` - Handle scope changes

## BMAD Phases

1. **Analysis** (Optional) - Brainstorming, research, product brief
2. **Planning** (Required) - Create PRD
3. **Solutioning** - Architecture, epics & stories
4. **Implementation** - Sprint planning, story development

## Resources

- [BMAD Documentation](https://docs.bmad-method.org/)
- [Workflow Map](https://docs.bmad-method.org/reference/workflow-map/)
- [GitHub](https://github.com/bmad-code-org/BMAD-METHOD)
- [Discord](https://discord.gg/gk8jAdXWmj)
