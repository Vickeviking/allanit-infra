# ===== Config =====
NETWORK=appnet
DEV_COMPOSE=docker-compose.dev.yml
PROD_COMPOSE=docker-compose.yml

.PHONY: ensure-network dev dev-down dev-logs prod-build prod-up prod-down prod-logs help

help:
	@echo "Targets:"
	@echo "  make dev         - Startar Vite i container (HMR) via $(DEV_COMPOSE)"
	@echo "  make dev-down    - Stoppar dev-containern"
	@echo "  make dev-logs    - Följer dev-loggar"
	@echo "  make prod-build  - Bygger nginx-imagen (prod)"
	@echo "  make prod-up     - Startar prod-dashboard på port 5173:80"
	@echo "  make prod-down   - Stoppar prod-dashboard"
	@echo "  make prod-logs   - Följer prod-loggar"

ensure-network:
	@docker network inspect $(NETWORK) >/dev/null 2>&1 || \
		( echo ">> Skapar nätverk $(NETWORK)"; docker network create $(NETWORK) )

# ===== Dev (Vite i container) =====
dev: ensure-network
	@docker compose -f $(DEV_COMPOSE) up

dev-down:
	@docker compose -f $(DEV_COMPOSE) down

dev-logs:
	@docker compose -f $(DEV_COMPOSE) logs -f dashboard-dev

# ===== Prod (nginx som serverar build) =====
prod-build:
	@docker compose -f $(PROD_COMPOSE) build dashboard

prod-up: ensure-network
	@docker compose -f $(PROD_COMPOSE) up -d dashboard

prod-down:
	@docker compose -f $(PROD_COMPOSE) down

prod-logs:
	@docker compose -f $(PROD_COMPOSE) logs -f dashboard

clean:
	@echo ">> Rensar lokala cache och moduler..."
	@docker compose -f $(DEV_COMPOSE) down -v --remove-orphans 2>/dev/null || true
	@rm -rf $(NODE_MODULES) $(CACHE_DIR) dist 2>/dev/null || true
	@echo "✅ Rensat!"
