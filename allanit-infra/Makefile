###############################################################################
# SwarmSync Makefile 
###############################################################################

# Variables
DC = docker compose
CORE = core

# Detect this file for help target
FILE := $(firstword $(MAKEFILE_LIST))

.PHONY: help menu build up-core up-all down \
        migrate db-save db-load db_store docker-clean \
        run-core  \
        test-unit test-integration test-all


#### Build & Up/Down ##########################################################
build: ## Build core
	@echo "▶ Building images..."
	$(DC) build $(CORE) $(CONSUMER) $(WORKER)

up-core: ## Start Core service + 
	@echo "▶ Starting Core"
	$(DC) up -d $(CORE) 
up-all: ## Start Core
	@echo "▶ Starting all services..."
	$(DC) up -d $(CORE) 

down: ## Stop all SwarmSync containers
	@echo "▶ Stopping all services..."
	@$(DC) down

docker-clean: down 
	@echo "▶ Pruning Docker system…"
	@docker system prune -af --volumes

#### Diesel tasks ###############################################################
migrate: up-core ## 
	@echo "▶ Applying migrations…"
	@$(DC) exec -T $(CORE) diesel migration run

#### Run targets #############################################################
run-core: ## Launch core binary
	$(require_core)
	@echo "▶ Running Core API…"
	@$(DC) exec -T $(CORE) cargo run --bin core

#### Testing #################################################################
test-unit: ## Run unit tests in Core, Consumer & Worker
	@echo "▶ Core unit tests…"
	@$(DC) exec $(CORE)        cargo test -- --include-ignored

test-integration: up-all migrate ## Run integration tests for every crate (host-side)
	$(require_core)
	@echo "▶ Core integration tests…"
	@cd allanit_infra/core       && cargo test -- --include-ignored

test-all: test-unit test-integration ## Run all tests

#### Help & Menu #############################################################
help: ## Show this help
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@grep -E '^[a-zA-Z0-9_-]+:.*?##' $(FILE) \
	  | sort \
	  | awk -F ':.*?## ' '{printf "  %-20s %s\n", $$1, $$2}'

menu: help ## Interactive menu
	@echo ""
	@grep -E '^[a-zA-Z0-9_-]+:.*?##' $(FILE) \
	  | sort \
	  | awk -F ':.*?## ' '{print NR ") " $$1 " — " $$2}' \
	  | tee /dev/tty \
	  | { read -p "Choose a target: " num </dev/tty; \
	      cmd=$$(grep -E '^[a-zA-Z0-9_-]+:.*?##' $(FILE) \
	             | sort \
	             | sed -n "$$num p" \
	             | awk -F ':.*?## ' '{print $$1}'); \
	      echo "▶ Running $$cmd…"; \
	      make $$cmd; }





