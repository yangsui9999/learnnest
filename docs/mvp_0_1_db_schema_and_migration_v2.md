# learnnest —— MVP 0.1 数据库字段说明表 & Migration 指南（v2）

> 版本：v2（修正：补齐 user_id、补齐 Plan/PlanTask，并统一表名为**单数**）  
> 技术栈：Rust + Salvo + PostgreSQL + Docker  
> 目标：为 MVP 0.1 提供**可直接写 migration**的数据库契约，确保多环境一致、避免返工。

---

## 0. 表名：为什么我现在统一用单数？你为什么会看到复数？

你说得对：我前面的回复里一会儿单数、一会儿复数，确实会让人晕。

- **推荐规范（本项目采用）**：表名用**单数**（`user`, `task`, `plan`, `resource`…）
- 你看到的 `tasks/plans` 属于我在示例里沿用了某些框架/社区“默认复数”的习惯写法，**但不是强制**。
- PostgreSQL 并不强制单数/复数；**关键是团队一致**。既然你已经倾向单数，我们就全程统一成单数，后续所有文档与 SQL 都按这个来。

> 本 v2 文档：**100% 单数表名**（含关联表也按单数命名）

---

## 1. MySQL 与 PostgreSQL 设计异同（与你关心的点对齐）

### 1.1 UUID 主键 vs BIGINT 自增
- PG 中 `UUID` 是一等公民类型，适合对外暴露、分布式、不可猜测。
- 本项目所有主键统一用 `UUID`。

### 1.2 NOT NULL + DEFAULT
- 仍是最佳实践；状态/审计字段强烈建议 `NOT NULL + DEFAULT`。

### 1.3 软删除
- 统一 `is_deleted BOOLEAN NOT NULL DEFAULT false`  
- 查询默认过滤 `is_deleted = false`。

---

## 2. ER 结构（MVP 0.1）

```
user 1 ──< task
user 1 ──< resource

plan N ──< plan_task >── N task
task N ──< task_resource >── N resource
task ──< task_relation >── task   （自关联：拆分/包含）
```

---

## 3. 字段说明表（MVP 0.1 定稿）

> 说明：  
> - 所有业务表包含审计字段：`created_at/created_by/updated_at/updated_by/is_deleted`  
> - 时间统一使用 `TIMESTAMPTZ`  
> - 枚举字段用 `TEXT + CHECK`（MVP 易演进）

---

### 3.1 user（用户）

| 字段 | 类型 | 约束 | 说明 |
|---|---|---|---|
| id | UUID | PK | 用户 ID |
| name | TEXT | NOT NULL | 显示名 |
| role | TEXT | NOT NULL DEFAULT 'parent' | parent / child（MVP 可先固定 parent） |
| created_at | TIMESTAMPTZ | NOT NULL DEFAULT now() | 创建时间 |
| created_by | UUID | NOT NULL | 创建人（MVP 可等于 id） |
| updated_at | TIMESTAMPTZ | NOT NULL DEFAULT now() | 更新时间 |
| updated_by | UUID | NOT NULL | 更新人（MVP 可等于 id） |
| is_deleted | BOOLEAN | NOT NULL DEFAULT false | 软删除 |

---

### 3.2 task（任务）

> 任务统一承载：作业/练习/预习/复习/计划（plan 作为 type 的一种）

| 字段 | 类型 | 约束 | 说明 |
|---|---|---|---|
| id | UUID | PK | 任务 ID |
| user_id | UUID | NOT NULL FK user(id) | **归属用户（实现用户隔离的关键）** |
| title | TEXT | NOT NULL | 标题 |
| description | TEXT |  | 描述 |
| type | TEXT | NOT NULL | homework / practice / preview / review / plan |
| subject | TEXT | NOT NULL | math / chinese / english / physics / chemistry / programming / other |
| status | TEXT | NOT NULL DEFAULT 'active' | active / done / archived |
| due_date | TIMESTAMPTZ |  | 截止时间 |
| completed_at | TIMESTAMPTZ |  | 完成时间（可选） |
| source | TEXT | NOT NULL DEFAULT 'manual' | manual / ocr / wechat / dingtalk / ai |
| created_at | TIMESTAMPTZ | NOT NULL DEFAULT now() | 创建时间 |
| created_by | UUID | NOT NULL | 创建人 |
| updated_at | TIMESTAMPTZ | NOT NULL DEFAULT now() | 更新时间 |
| updated_by | UUID | NOT NULL | 更新人 |
| is_deleted | BOOLEAN | NOT NULL DEFAULT false | 软删除 |

CHECK 建议（MVP 就加）：
- `type IN ('homework','practice','preview','review','plan')`
- `subject IN ('math','chinese','english','physics','chemistry','programming','other')`
- `status IN ('active','done','archived')`
- `source IN ('manual','ocr','wechat','dingtalk','ai')`

索引建议：
- `task(user_id)`
- `task(user_id, status)`
- `task(user_id, due_date)`

---

### 3.3 plan（计划壳子）

> MVP 0.1：只做“计划基本信息 + 关联任务”，不做复杂进度算法

| 字段 | 类型 | 约束 | 说明 |
|---|---|---|---|
| id | UUID | PK | 计划 ID |
| user_id | UUID | NOT NULL FK user(id) | 归属用户 |
| title | TEXT | NOT NULL | 计划名称 |
| description | TEXT |  | 描述 |
| status | TEXT | NOT NULL DEFAULT 'active' | active / archived |
| created_at | TIMESTAMPTZ | NOT NULL DEFAULT now() | 创建时间 |
| created_by | UUID | NOT NULL | 创建人 |
| updated_at | TIMESTAMPTZ | NOT NULL DEFAULT now() | 更新时间 |
| updated_by | UUID | NOT NULL | 更新人 |
| is_deleted | BOOLEAN | NOT NULL DEFAULT false | 软删除 |

CHECK 建议：
- `status IN ('active','archived')`

索引建议：
- `plan(user_id, status)`

---

### 3.4 plan_task（计划-任务 关联表）

| 字段 | 类型 | 约束 | 说明 |
|---|---|---|---|
| plan_id | UUID | NOT NULL FK plan(id) | 计划 |
| task_id | UUID | NOT NULL FK task(id) | 任务 |
| created_at | TIMESTAMPTZ | NOT NULL DEFAULT now() | 创建时间 |
| created_by | UUID | NOT NULL | 创建人 |

主键建议：
- `PRIMARY KEY (plan_id, task_id)`

索引建议：
- `plan_task(task_id)`（反查某任务在哪些计划中）

---

### 3.5 resource（资源）

| 字段 | 类型 | 约束 | 说明 |
|---|---|---|---|
| id | UUID | PK | 资源 ID |
| user_id | UUID | NOT NULL FK user(id) | **归属用户（实现用户隔离的关键）** |
| file_name | TEXT | NOT NULL | 原始文件名 |
| file_type | TEXT | NOT NULL | pdf / image / doc / other |
| storage_url | TEXT | NOT NULL | OSS 地址 |
| extracted_text | TEXT |  | AI/LLM 提取出的文本（可选） |
| created_at | TIMESTAMPTZ | NOT NULL DEFAULT now() | 创建时间 |
| created_by | UUID | NOT NULL | 上传人 |
| updated_at | TIMESTAMPTZ | NOT NULL DEFAULT now() | 更新时间 |
| updated_by | UUID | NOT NULL | 更新人 |
| is_deleted | BOOLEAN | NOT NULL DEFAULT false | 软删除 |

CHECK 建议：
- `file_type IN ('pdf','image','doc','other')`

索引建议：
- `resource(user_id, created_at)`

---

### 3.6 task_resource（任务-资源 关联表）

| 字段 | 类型 | 约束 | 说明 |
|---|---|---|---|
| task_id | UUID | NOT NULL FK task(id) | 任务 |
| resource_id | UUID | NOT NULL FK resource(id) | 资源 |
| created_at | TIMESTAMPTZ | NOT NULL DEFAULT now() | 创建时间 |
| created_by | UUID | NOT NULL | 创建人 |

主键建议：
- `PRIMARY KEY (task_id, resource_id)`

索引建议：
- `task_resource(resource_id)`（反查资源关联了哪些任务）

---

### 3.7 task_relation（任务自关联：拆分/包含）

> 用于：AI 拆分作业 / 包含关系

| 字段 | 类型 | 约束 | 说明 |
|---|---|---|---|
| parent_task_id | UUID | NOT NULL FK task(id) | 父任务 |
| child_task_id | UUID | NOT NULL FK task(id) | 子任务 |
| relation_type | TEXT | NOT NULL | plan / ai_split / manual |
| created_at | TIMESTAMPTZ | NOT NULL DEFAULT now() | 创建时间 |
| created_by | UUID | NOT NULL | 创建人 |

主键建议：
- `PRIMARY KEY (parent_task_id, child_task_id, relation_type)`

CHECK 建议：
- `relation_type IN ('plan','ai_split','manual')`

索引建议：
- `task_relation(parent_task_id)`
- `task_relation(child_task_id)`

---

## 4. Migration（必须写进文档：是的）

### 4.1 为什么要写 Migration？
- 数据库结构会演进；migration 是“结构版本控制”
- 多环境（本地/测试/生产）一致性靠它保证
- Rust 强类型项目，越早用 migration 越省返工

### 4.2 推荐工具：sqlx-cli
示例流程：

```bash
cargo install sqlx-cli
sqlx migrate add 0001_init
sqlx migrate run
sqlx migrate revert
```

### 4.3 目录建议

```
backend/
  migrations/
    0001_init.sql
    0002_indexes.sql
```

约定：
- 一次 migration 只做一类变更（建表 / 加索引 / 改字段）
- 建表阶段就加 CHECK/主键/唯一约束
- 索引可独立 migration（更清晰）

---

## 5. 下一步（推荐顺序）

1. 依据本字段说明表写 `0001_init` migration（建表 + CHECK + 主键/唯一约束）
2. 写 `0002_indexes` migration（补充索引）
3. Rust（SQLx）数据模型 struct 与查询模板
4. API 清单（按页面反推）
