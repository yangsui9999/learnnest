# ==================================
# LearnNest - Makefile
# 简化常用命令
# ==================================

.PHONY: dev prod down logs build clean db-shell help

# 默认目标
help:
	@echo "LearnNest 常用命令："
	@echo ""
	@echo "  make dev        启动开发环境"
	@echo "  make prod       启动生产环境"
	@echo "  make down       停止所有服务"
	@echo "  make logs       查看日志"
	@echo "  make build      重新构建镜像"
	@echo "  make db-shell   进入数据库 Shell"
	@echo "  make clean      清理容器和镜像"
	@echo ""

# 开发环境
dev:
	docker compose -f deploy/compose.yml -f deploy/compose.dev.yml up -d
	@echo "开发环境已启动"
	@echo "  API: http://localhost:8080"
	@echo "  DB:  localhost:5432"

# 生产环境
prod:
	docker compose -f deploy/compose.yml -f deploy/compose.prod.yml up -d
	@echo "生产环境已启动"

# 停止服务
down:
	docker compose -f deploy/compose.yml down

# 查看日志
logs:
	docker compose -f deploy/compose.yml logs -f

# 查看特定服务日志
logs-api:
	docker compose -f deploy/compose.yml logs -f api

logs-db:
	docker compose -f deploy/compose.yml logs -f db

# 重新构建
build:
	docker compose -f deploy/compose.yml build --no-cache

# 进入数据库 Shell
db-shell:
	docker compose -f deploy/compose.yml exec db psql -U learnnest -d learnnest

# 清理
clean:
	docker compose -f deploy/compose.yml down -v --rmi local
	@echo "已清理容器、卷和本地镜像"

# 数据库迁移（开发环境）
migrate:
	docker compose -f deploy/compose.yml exec api sqlx migrate run

# 前端开发
frontend-dev:
	cd frontend && npm run dev

# 后端开发（本地，不用 Docker）
backend-dev:
	cd backend && cargo watch -x run
