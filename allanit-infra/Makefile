###############################################################################
# SwarmSync Makefile 
###############################################################################

DC = docker compose
CORE = core
FILE := $(firstword $(MAKEFILE_LIST))

.PHONY: help menu build up-core up-all down \
        migrate db-save db-load db_store docker-clean \
        logs-core \
        test-unit test-integration test-all

#### Build & Up/Down ##########################################################
build: ## Build core
	@echo "▶ Building images..."
	$(DC) build $(CORE)

up-core: ## Start Core service
	@echo "▶ Starting Core..."
	$(DC) up -d $(CORE)

up-all: ## Start all services
	@echo "▶ Starting all services..."
	$(DC) up -d

down: ## Stop all containers
	@echo "▶ Stopping all services..."
	$(DC) down

docker-clean: down ## Full prune
	@echo "▶ Pruning Docker system..."
	docker system prune -af --volumes

#### Diesel tasks ###############################################################
migrate: up-core ## Apply migrations
	@echo "▶ Applying migrations..."
	$(DC) exec -T $(CORE) diesel migration run

#### Logs ####################################################################
logs-core: ## Follow Core logs
	@echo "▶ Attaching to Core logs..."
	$(DC) logs -f $(CORE)

#### Testing #################################################################
test-unit: ## Run unit tests
	@echo "▶ Core unit tests..."
	$(DC) exec $(CORE) cargo test -- --include-ignored

test-integration: up-all migrate ## Run integration tests
	@echo "▶ Core integration tests..."
	cd allanit_infra/core && cargo test -- --include-ignored

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
