# Traefik Specification

## Overview

| Attribute | Value |
|-----------|-------|
| **Technology** | Traefik Open Source |
| **Purpose** | API Gateway, routing, TLS |
| **Ports** | 80 (HTTP), 443 (HTTPS), 8080 (dashboard) |

---

## Why Traefik

| Requirement | Traefik Capability |
|-------------|-------------------|
| Forward auth | ✅ Built-in middleware |
| Docker labels | ✅ Auto-discovery |
| TLS (Let's Encrypt) | ✅ ACME support |
| Simple config | ✅ YAML or labels |
| Open source | ✅ No license fees |

---

## Routing

### Production Routes

| Route | Backend | Auth |
|-------|---------|------|
| `/auth/*` | Auth Service | No |
| `/api/*` | API Service | Yes (forward auth) |
| `/events` | API Service | Yes |
| `/*` (static) | Frontend | No |

### Forward Auth Flow

```
1. Request arrives: GET /api/books
         │
         ▼
2. Traefik calls Auth Service: GET /verify
   Headers forwarded:
   - Authorization: Bearer <jwt>
   - X-Forwarded-Uri: /api/books
   - X-Forwarded-Method: GET
         │
         ▼
3a. Auth Service returns 200 + headers:
    - X-User-Id: user-uuid
    - X-User-Email: user@example.com
    - X-User-Tier: pro
         │
         ▼
4a. Traefik forwards to API Service with user headers

--- OR ---

3b. Auth Service returns 401
         │
         ▼
4b. Traefik returns 401 to client
```

---

## Configuration

### Static Config (traefik.yml)

```yaml
api:
  dashboard: true
  insecure: true  # Dev only

entryPoints:
  web:
    address: ":80"
    http:
      redirections:
        entryPoint:
          to: websecure
          scheme: https
  websecure:
    address: ":443"

providers:
  docker:
    endpoint: "unix:///var/run/docker.sock"
    exposedByDefault: false
  file:
    filename: /etc/traefik/dynamic.yml

certificatesResolvers:
  letsencrypt:
    acme:
      email: admin@disona.app
      storage: /letsencrypt/acme.json
      httpChallenge:
        entryPoint: web
```

### Dynamic Config (dynamic.yml)

```yaml
http:
  middlewares:
    auth-forward:
      forwardAuth:
        address: "http://auth-service:8081/verify"
        trustForwardHeader: true
        authResponseHeaders:
          - "X-User-Id"
          - "X-User-Email"
          - "X-User-Tier"
          - "X-User-Quota-Remaining"
    
    rate-limit:
      rateLimit:
        average: 100
        burst: 50
    
    cors:
      headers:
        accessControlAllowMethods:
          - GET
          - POST
          - PUT
          - DELETE
          - OPTIONS
        accessControlAllowHeaders:
          - Authorization
          - Content-Type
        accessControlAllowOriginList:
          - https://app.disona.app
        accessControlMaxAge: 86400
```

---

## Docker Labels

### Auth Service

```yaml
auth-service:
  labels:
    - "traefik.enable=true"
    - "traefik.http.routers.auth.rule=PathPrefix(`/auth`)"
    - "traefik.http.routers.auth.entrypoints=websecure"
    - "traefik.http.routers.auth.tls.certresolver=letsencrypt"
    - "traefik.http.services.auth.loadbalancer.server.port=8081"
```

### API Service

```yaml
api-service:
  labels:
    - "traefik.enable=true"
    - "traefik.http.routers.api.rule=PathPrefix(`/api`)"
    - "traefik.http.routers.api.entrypoints=websecure"
    - "traefik.http.routers.api.tls.certresolver=letsencrypt"
    - "traefik.http.routers.api.middlewares=auth-forward,cors"
    - "traefik.http.services.api.loadbalancer.server.port=8080"
```

### Frontend

```yaml
frontend:
  labels:
    - "traefik.enable=true"
    - "traefik.http.routers.frontend.rule=PathPrefix(`/`)"
    - "traefik.http.routers.frontend.entrypoints=websecure"
    - "traefik.http.routers.frontend.tls.certresolver=letsencrypt"
    - "traefik.http.routers.frontend.priority=1"  # Lowest priority
    - "traefik.http.services.frontend.loadbalancer.server.port=3000"
```

---

## Development Config

```yaml
# docker-compose.yml
traefik:
  image: traefik:v3.0
  command:
    - "--api.insecure=true"
    - "--providers.docker=true"
    - "--providers.docker.exposedbydefault=false"
    - "--entrypoints.web.address=:80"
  ports:
    - "80:80"
    - "8080:8080"  # Dashboard
  volumes:
    - /var/run/docker.sock:/var/run/docker.sock:ro
```

---

## Security Headers

```yaml
http:
  middlewares:
    security-headers:
      headers:
        frameDeny: true
        contentTypeNosniff: true
        browserXssFilter: true
        referrerPolicy: "strict-origin-when-cross-origin"
        customResponseHeaders:
          X-Robots-Tag: "noindex,nofollow"
```

---

## Health Checks

```yaml
http:
  services:
    api:
      loadBalancer:
        servers:
          - url: "http://api-service:8080"
        healthCheck:
          path: /health
          interval: 10s
          timeout: 3s
```

---

## Monitoring

### Dashboard

Access at `http://localhost:8080` (dev only)

### Metrics

```yaml
metrics:
  prometheus:
    addServicesLabels: true
    addEntryPointsLabels: true
```

### Logs

```yaml
log:
  level: INFO
  format: json

accessLog:
  format: json
  fields:
    headers:
      names:
        X-User-Id: keep
```

---

## Rate Limiting

### Global Rate Limit

```yaml
http:
  middlewares:
    rate-limit:
      rateLimit:
        average: 100  # requests per second
        burst: 50     # allow bursts
```

### Per-User Rate Limit

Handled by Auth Service (more granular control).

---

## Production Checklist

| Item | Status |
|------|--------|
| TLS certificates (Let's Encrypt) | Required |
| Dashboard disabled or auth-protected | Required |
| Access logs enabled | Required |
| Security headers | Required |
| Rate limiting | Required |
| Health checks | Required |
