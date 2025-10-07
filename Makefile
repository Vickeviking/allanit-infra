###############################################################################
# Allanit — Top-level Makefile (controls infra + mock)
###############################################################################

SHELL := /bin/bash
DC    := docker compose

# Paths
INFRA_DIR          := allanit-infra
INFRA_COMPOSE      := $(INFRA_DIR)/docker-compose.yml
MOCK_DIR           := seventimes-mock
MOCK_COMPOSE       := $(MOCK_DIR)/docker-compose.yml

# Service names (as defined inside each compose)
INFRA_CORE_SVC     := core
INFRA_DB_SVC       := postgres
MOCK_DB_SVC        := db
MOCK_API_SVC       := seventimes-mock

# Backups (fix: separate dirs per stack to avoid target override)
INFRA_BACKUPS_DIR  := $(INFRA_DIR)/backups
MOCK_BACKUPS_DIR   := $(MOCK_DIR)/backups
INFRA_BACKUP_FILE  := $(INFRA_BACKUPS_DIR)/infra_manual_backup.sql
MOCK_BACKUP_FILE   := $(MOCK_BACKUPS_DIR)/manual_backup.sql

FILE := $(firstword $(MAKEFILE_LIST))

.PHONY: help \
        build-all up-all down-all logs-all \
        build-infra up-infra down-infra logs-infra \
        build-mock up-mock down-mock logs-mock \
        mock-health mock-list-customers mock-list-pos \
        mock-db-save mock-db-load mock-db-shell mock-db-reset \
        migrate-infra run-core test-infra \
        infra-db-shell infra-db-shell-docker infra-db-pgcli open-dbeaver \
        infra-db-url infra-db-wait infra-db-save infra-db-load infra-db-reset

## ---------------------------------------------------------------------------
## Top-level orchestration
## ---------------------------------------------------------------------------

build-all: build-infra build-mock ## Build images for infra + mock
	@echo "✅ build-all done"

up-all: up-infra up-mock ## Start infra + mock stacks
	@echo "✅ up-all done"

down-all: ## Stop both stacks (keeps volumes)
	$(DC) -f $(INFRA_COMPOSE) down
	$(DC) -f $(MOCK_COMPOSE)  down
	@echo "✅ down-all done"

logs-all: ## Tail logs from both stacks
	@echo "▶ Infra logs"; $(DC) -f $(INFRA_COMPOSE) logs -f &
	@echo "▶ Mock logs";  $(DC) -f $(MOCK_COMPOSE)  logs -f

## ---------------------------------------------------------------------------
## Infra stack (allanit-infra/)
## ---------------------------------------------------------------------------

build-infra: ## Build infra images
	$(DC) -f $(INFRA_COMPOSE) build

up-infra: ## Start infra stack
	$(DC) -f $(INFRA_COMPOSE) up -d

down-infra: ## Stop infra stack
	$(DC) -f $(INFRA_COMPOSE) down

logs-infra: ## Tail infra logs
	$(DC) -f $(INFRA_COMPOSE) logs -f

# Optional helpers (only work if you have a core service in infra compose)
migrate-infra: up-infra ## Run Diesel migrations inside core (if present)
	$(DC) -f $(INFRA_COMPOSE) exec -T $(INFRA_CORE_SVC) diesel migration run

test-infra: ## cargo test inside infra container (if present)
	$(DC) -f $(INFRA_COMPOSE) exec -T $(INFRA_CORE_SVC) cargo test -- --include-ignored

# Psql-shell via docker compose (service-namnet i compose)
infra-db-shell: ## psql shell into infra DB (compose service: $(INFRA_DB_SVC))
	$(DC) -f $(INFRA_COMPOSE) exec -it $(INFRA_DB_SVC) \
	  psql -U postgres -d app_db

infra-db-shell-docker: ## psql via docker exec (container_name=allanit_infra_db)
	docker exec -it allanit_infra_db psql -U postgres -d app_db

infra-db-wait: ## Wait until infra DB accepts connections
	$(DC) -f $(INFRA_COMPOSE) exec -T $(INFRA_DB_SVC) \
	  sh -lc 'until pg_isready -U postgres -d app_db -h localhost; do sleep 0.5; done; echo "✅ DB ready"'

infra-db-url: ## Print connection URLs (inside compose vs host)
	@echo "Inside compose network: postgres://postgres:postgres@postgres:5432/app_db"
	@echo "From host (requires ports: 5432:5432): postgres://postgres:postgres@localhost:5432/app_db"

infra-db-pgcli: ## pgcli to infra DB from host (Arch: sudo pacman -S pgcli)
	pgcli postgresql://postgres:postgres@localhost:5432/app_db

open-dbeaver: ## launch DBeaver GUI
	dbeaver &

# --- Backups: create dirs (fixed: do NOT define same path twice) ---
$(INFRA_BACKUPS_DIR):
	mkdir -p $@

$(MOCK_BACKUPS_DIR):
	mkdir -p $@

# --- Infra DB dump/restore ---
infra-db-save: $(INFRA_BACKUPS_DIR) ## Dump infra DB → allanit-infra/backups/infra_manual_backup.sql
	$(DC) -f $(INFRA_COMPOSE) exec -T $(INFRA_DB_SVC) \
	  pg_dump -U postgres app_db > $(INFRA_BACKUP_FILE)
	@echo "✅ Saved to $(INFRA_BACKUP_FILE)"

infra-db-load: ## Restore infra DB from backup file
	@test -f $(INFRA_BACKUP_FILE) || (echo "Missing $(INFRA_BACKUP_FILE)"; exit 1)
	cat $(INFRA_BACKUP_FILE) | $(DC) -f $(INFRA_COMPOSE) exec -T $(INFRA_DB_SVC) \
	  psql -U postgres -d app_db
	@echo "✅ Restored from $(INFRA_BACKUP_FILE)"

infra-db-reset: ## Drop infra DB volume & re-up (⚠ wipes data)
	$(DC) -f $(INFRA_COMPOSE) down
	docker volume rm $$(docker volume ls -q | grep allanit-infra_postgres_data) || true
	$(DC) -f $(INFRA_COMPOSE) up -d

# ---- schema generation ----
schema:
	cd allanit-infra/allanit_infra/common && \
	export DATABASE_URL=postgres://postgres:postgres@localhost:5432/app_db && \
	diesel print-schema > src/database/schema.rs && \
	echo "✅ schema.rs uppdaterad (lokalt)"



## ---------------------------------------------------------------------------
## Mock stack (seventimes-mock/)
## ---------------------------------------------------------------------------

build-mock: ## Build mock images
	$(DC) -f $(MOCK_COMPOSE) build

up-mock: ## Start mock stack (DB + API)
	$(DC) -f $(MOCK_COMPOSE) up -d

down-mock: ## Stop mock stack
	$(DC) -f $(MOCK_COMPOSE) down

logs-mock: ## Tail mock logs
	$(DC) -f $(MOCK_COMPOSE) logs -f

# Mock API quick calls
mock-health: ## GET /health
	curl -s http://localhost:4000/health | jq .

mock-list-customers: ## GET /api/v1/customers
	curl -s -H "Client-Secret: dev-secret" http://localhost:4000/api/v1/customers | jq .

mock-list-pos: ## GET /api/v1/purchase-orders
	curl -s -H "Client-Secret: dev-secret" http://localhost:4000/api/v1/purchase-orders | jq .

# --- Mock DB helpers ---
mock-db-save: $(MOCK_BACKUPS_DIR) ## Dump mock DB → seventimes-mock/backups/manual_backup.sql
	$(DC) -f $(MOCK_COMPOSE) exec -T $(MOCK_DB_SVC) \
	  pg_dump -U sev sev_mock > $(MOCK_BACKUP_FILE)
	@echo "✅ Saved to $(MOCK_BACKUP_FILE)"

mock-db-load: ## Restore mock DB from backup file
	@test -f $(MOCK_BACKUP_FILE) || (echo "Missing $(MOCK_BACKUP_FILE)"; exit 1)
	cat $(MOCK_BACKUP_FILE) | $(DC) -f $(MOCK_COMPOSE) exec -T $(MOCK_DB_SVC) \
	  psql -U sev -d sev_mock
	@echo "✅ Restored from $(MOCK_BACKUP_FILE)"

mock-db-shell: ## psql shell into mock DB
	$(DC) -f $(MOCK_COMPOSE) exec -it $(MOCK_DB_SVC) psql -U sev -d sev_mock

mock-db-reset: ## Drop mock DB volume & re-up (⚠ wipes data)
	$(DC) -f $(MOCK_COMPOSE) down
	docker volume rm $$(docker volume ls -q | grep seventimes-mock_dbdata) || true
	$(DC) -f $(MOCK_COMPOSE) up -d

## ---------------------------------------------------------------------------
## Help
## ---------------------------------------------------------------------------

help: ## Show this help
	@echo "Usage: make <target>"
	@echo ""
	@echo "Top-level targets:"
	@grep -E '^[a-zA-Z0-9_-]+:.*?##' $(FILE) \
	  | sed -n 's/:.*##/:/p' \
	  | awk -F ':' '{printf "  %-22s %s\n", $$1, $$2}' \
	  | sort
