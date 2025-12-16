# 技术实施指南

> 本文档是 tech-list.md 的详细实施版本，记录已确定议题的具体方案。
> 状态：持续更新中

---

## 一、多环境配置（#33-36 ✅ 已确定）

### 1.1 核心原则

**12-Factor App 配置原则：**
- 应用程序只从环境变量读取配置
- 配置与代码严格分离
- 同一份代码可以部署到任何环境

### 1.2 目录结构

```
learnnest/
├── backend/
│   ├── .env.local                # 后端本地敏感配置（不提交）
│   ├── config/
│   │   ├── base.env              # 共享默认值（提交）
│   │   ├── development.env       # 开发环境（提交）
│   │   └── production.env        # 生产环境非敏感配置（提交）
│   ├── Dockerfile
│   └── src/
│
├── frontend/
│   ├── .env.local                # 前端本地敏感配置（不提交）
│   ├── .env                      # 前端默认值（提交）
│   ├── .env.development          # 前端开发环境（提交）
│   ├── .env.production           # 前端生产环境（提交）
│   └── src/
│
├── deploy/
│   ├── compose.yml               # Docker 基础服务定义
│   ├── compose.dev.yml           # 开发环境覆盖
│   ├── compose.prod.yml          # 生产环境覆盖
│   └── nginx/
│       ├── nginx.dev.conf
│       └── nginx.prod.conf
│
└── Makefile                      # 简化命令
```

### 1.3 配置分层与优先级

**后端配置优先级（高 → 低）：**
```
Shell 环境变量 > .env.local > development.env > base.env
```

**前端配置优先级（Vite 约定）：**
```
.env.local > .env.[mode] > .env
```

### 1.4 配置文件内容划分

| 文件 | 内容 | 是否提交 |
|------|------|----------|
| base.env | 所有环境共享的默认值（端口、超时等） | ✓ 提交 |
| development.env | 开发环境特定配置（debug 日志、宽松限制） | ✓ 提交 |
| production.env | 生产环境非敏感配置（日志级别、CORS 域名） | ✓ 提交 |
| .env.local | 本地敏感配置（数据库密码、API 密钥） | ✗ 不提交 |

### 1.5 后端配置加载（Rust）

```rust
// src/main.rs
fn main() {
    // 按优先级从低到高加载（dotenvy 不覆盖已存在的变量）
    dotenvy::from_filename("config/base.env").ok();
    dotenvy::from_filename("config/development.env").ok();
    dotenvy::from_path(".env.local").ok();

    // 应用只从环境变量读取
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
}
```

### 1.6 敏感信息管理

| 环境 | 敏感信息来源 |
|------|-------------|
| 本地开发 | .env.local 文件 |
| 生产环境 | GitHub Secrets / 服务器环境变量 |

**生产部署命令：**
```bash
docker compose \
  -f deploy/compose.yml \
  -f deploy/compose.prod.yml \
  --env-file /opt/learnnest/.env.secrets \
  up -d
```

---

## 二、部署架构（#40-41 ✅ 已确定）

### 2.1 Docker Compose 分层

**compose.yml（基础服务定义）：**
```yaml
services:
  api:
    build:
      context: ../backend
      dockerfile: Dockerfile
    depends_on:
      - db

  db:
    image: postgres:15
    volumes:
      - postgres_data:/var/lib/postgresql/data

  nginx:
    image: nginx:alpine
    depends_on:
      - api

volumes:
  postgres_data:
```

**compose.dev.yml（开发环境覆盖）：**
```yaml
services:
  api:
    env_file:
      - ../backend/config/base.env
      - ../backend/config/development.env
      - ../backend/.env.local
    ports:
      - "8080:8080"
    volumes:
      - ../backend/src:/app/src  # 热重载

  nginx:
    volumes:
      - ./nginx/nginx.dev.conf:/etc/nginx/nginx.conf:ro
    ports:
      - "80:80"
```

**compose.prod.yml（生产环境覆盖）：**
```yaml
services:
  api:
    env_file:
      - ../backend/config/base.env
      - ../backend/config/production.env
    restart: always

  nginx:
    volumes:
      - ./nginx/nginx.prod.conf:/etc/nginx/nginx.conf:ro
      - ../frontend/dist:/usr/share/nginx/html:ro
    ports:
      - "80:80"
      - "443:443"
    restart: always
```

### 2.2 Nginx 配置

**nginx.prod.conf：**
```nginx
server {
    listen 80;
    server_name learnnest.example.com;

    # 前端静态资源
    location / {
        root /usr/share/nginx/html;
        try_files $uri $uri/ /index.html;
    }

    # API 代理
    location /api/ {
        proxy_pass http://api:8080/api/;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }
}
```

### 2.3 Makefile 简化命令

```makefile
.PHONY: dev prod down logs

dev:
	docker compose -f deploy/compose.yml -f deploy/compose.dev.yml up -d

prod:
	docker compose -f deploy/compose.yml -f deploy/compose.prod.yml up -d

down:
	docker compose -f deploy/compose.yml down

logs:
	docker compose -f deploy/compose.yml logs -f
```

---

## 三、API 设计规范（#1-6 待实施）

> 已在 tech-list.md 确定方向，待开发时详细补充

### 3.1 MVP 默认选择

| 议题 | 方案 |
|------|------|
| API 风格 | RESTful，`/api/tasks`、`/api/resources` |
| 统一响应格式 | `{ "code": 0, "data": {...}, "message": "ok" }` |
| 错误码 | HTTP 状态码为主，业务错误用 code 区分 |
| 分页 | `?page=1&size=20`，返回 `{ list, total, page, size }` |
| API 版本 | MVP 不加版本号 |
| 时间格式 | ISO 8601，统一 UTC |

---

## 四、认证与安全（#7-11 待实施）

### 4.1 MVP 默认选择

| 议题 | 方案 |
|------|------|
| 认证方案 | JWT，过期时间 7 天，MVP 不做刷新 |
| 权限模型 | 简单 owner 校验（task.account_id == 当前用户） |
| 数据隔离 | 按 account_id 隔离 |
| 密码加密 | Argon2 |
| CORS | 开发环境允许所有，生产环境限定域名 |

---

## 五、后端架构（#12-17 待实施）

### 5.1 MVP 默认选择

| 议题 | 方案 |
|------|------|
| 分层 | handler → service → repository |
| 错误处理 | 统一 AppError 枚举 |
| 数据校验 | handler 层用 validator |
| 日志 | tracing 库，INFO 级别 |
| 数据库连接池 | sqlx 内置，默认配置 |
| 事务 | service 层控制 |

---

## 版本记录

| 日期 | 变更 |
|------|------|
| 2025-12-16 | 创建文档，完成多环境配置、部署架构章节 |
