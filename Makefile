
###############################################################################
# Allanit — Top-level Makefile (controls infra + mock)
###############################################################################

SHELL := /bin/bash
DC    := docker compose

# Paths
INFRA_DIR      := allanit-infra
INFRA_COMPOSE  := $(INFRA_DIR)/docker-compose.yml
MOCK_DIR       := seventimes-mock
MOCK_COMPOSE   := $(MOCK_DIR)/docker-compose.yml

# Service names (as defined inside each compose)
# Adjust if you change service names in the compose files.
INFRA_CORE_SVC := core              
INFRA_DB_SVC   := postgres               
MOCK_DB_SVC    := db
MOCK_API_SVC   := seventimes-mock

# Mock DB creds (match your seventimes-mock/docker-compose.yml)
MOCK_DB_USER := sev
MOCK_DB_NAME := sev_mock
BACKUPS_DIR  := $(INFRA_DIR)/backups
BACKUP_FILE  := $(BACKUPS_DIR)/manual_backup.sql

FILE := $(firstword $(MAKEFILE_LIST))

.PHONY: help \
        build-all up-all down-all logs-all \
        build-infra up-infra down-infra logs-infra \
        build-mock up-mock down-mock logs-mock \
        mock-health mock-list-customers mock-list-pos \
        mock-db-save mock-db-load mock-db-shell mock-db-reset \
        migrate-infra run-core test-infra

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

run-core: ## cargo run core inside infra container (if present)
	$(DC) -f $(INFRA_COMPOSE) exec -T $(INFRA_CORE_SVC) cargo run -p core --bin core

test-infra: ## cargo test inside infra container (if present)
	$(DC) -f $(INFRA_COMPOSE) exec -T $(INFRA_CORE_SVC) cargo test -- --include-ignored

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

# Mock DB helpers
$(BACKUPS_DIR):
	mkdir -p $(BACKUPS_DIR)

mock-db-save: $(BACKUPS_DIR) ## Dump mock DB → allanit-infra/backups/manual_backup.sql
	$(DC) -f $(MOCK_COMPOSE) exec -T $(MOCK_DB_SVC) pg_dump -U $(MOCK_DB_USER) $(MOCK_DB_NAME) > $(BACKUP_FILE)
	@echo "✅ Saved to $(BACKUP_FILE)"

mock-db-load: ## Restore mock DB from backup file
	@test -f $(BACKUP_FILE) || (echo "Missing $(BACKUP_FILE)"; exit 1)
	cat $(BACKUP_FILE) | $(DC) -f $(MOCK_COMPOSE) exec -T $(MOCK_DB_SVC) psql -U $(MOCK_DB_USER) -d $(MOCK_DB_NAME)
	@echo "✅ Restored from $(BACKUP_FILE)"

mock-db-shell: ## psql shell into mock DB
	$(DC) -f $(MOCK_COMPOSE) exec -it $(MOCK_DB_SVC) psql -U $(MOCK_DB_USER) -d $(MOCK_DB_NAME)

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
