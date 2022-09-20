.DEFAULT_GOAL = help

## —— 📦 The amazing Makefile 📦 —————————————————————————————————————————
help: ## Outputs this help screen
	@grep -E '(^[a-zA-Z0-9_-]+:.*?##.*$$)|(^##)' Makefile | awk 'BEGIN {FS = ":.*?## "}{printf "\033[32m%-30s\033[0m %s\n", $$1, $$2}' | sed -e 's/\[32m##/[33m/'

## —— 🐋 Docker 🐋 —————————————————————————————————————————
build: ## Build the docker environment
	docker-compose build --no-cache

up: ## Up the docker environment
	docker-compose up -d --remove-orphans

down: ## Down the docker environment
	docker-compose down

## ——  Rust container  —————————————————————————————————————————
enter: ## Access to container
	docker exec -u container-user -ti rust-gs /usr/bin/fish

