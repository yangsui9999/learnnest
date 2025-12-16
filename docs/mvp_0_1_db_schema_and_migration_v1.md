# 学习管理系统 MVP 0.1 —— 数据库字段说明与 Migration 指南

> 本文档用于 **正式进入开发阶段**，统一数据库设计认知，避免 MySQL → PostgreSQL + UUID 转型过程中的设计分歧。
> 技术栈：Rust + Salvo + PostgreSQL + Docker

---

## 一、MySQL 与 PostgreSQL 在表设计上的核心异同

### 1. 主键设计差异

| 项目         | MySQL 常见做法        | PostgreSQL 推荐做法 |
| ------------ | --------------------- | ------------------- |
| 主键类型     | BIGINT AUTO_INCREMENT | UUID                |
| 主键生成     | 数据库生成            | 数据库或应用生成    |
| 是否对外暴露 | 通常不                | 可以安全对外        |

**结论**：  
UUID 在 PG 体系下已经同时满足：唯一性、安全性、分布式友好性。

---

### 2. NOT NULL + DEFAULT 是否仍是最佳实践？

是，而且 **比 MySQL 更重要**。

- 必填业务字段：`NOT NULL`
- 状态字段：`NOT NULL + DEFAULT`
- 审计字段：全部 `NOT NULL`

---

### 3. 软删除策略

统一使用：

```sql
is_deleted BOOLEAN NOT NULL DEFAULT false
```

MVP 阶段不引入 `deleted_at`。

---

### 4. 表名命名规范

- 使用 **单数表名**
- 与 Rust struct / DDD 模型保持一致

---

## 二、MVP 0.1 表结构字段说明

---

### users（用户表）

| 字段       | 类型        | 约束                   | 说明           |
| ---------- | ----------- | ---------------------- | -------------- |
| id         | UUID        | PK                     | 用户 ID        |
| name       | TEXT        | NOT NULL               | 昵称           |
| role       | TEXT        | NOT NULL               | parent / child |
| created_at | TIMESTAMPTZ | NOT NULL DEFAULT now() | 创建时间       |
| created_by | UUID        | NOT NULL               | 创建人         |
| updated_at | TIMESTAMPTZ | NOT NULL DEFAULT now() | 更新时间       |
| updated_by | UUID        | NOT NULL               | 更新人         |
| is_deleted | BOOLEAN     | NOT NULL DEFAULT false | 软删除         |

---

### tasks（任务表）

统一承载：作业 / 练习 / 预习 / 复习 / 计划

| 字段        | 类型        | 约束                      | 说明                                                                 |
| ----------- | ----------- | ------------------------- | -------------------------------------------------------------------- |
| id          | UUID        | PK                        | 任务 ID                                                              |
| title       | TEXT        | NOT NULL                  | 标题                                                                 |
| description | TEXT        |                           | 描述                                                                 |
| type        | TEXT        | NOT NULL                  | homework / practice / preview / review / plan                        |
| subject     | TEXT        | NOT NULL                  | math / chinese / english / physics / chemistry / programming / other |
| status      | TEXT        | NOT NULL DEFAULT 'active' | active / done / archived                                             |
| due_date    | TIMESTAMPTZ |                           | 截止时间                                                             |
| created_at  | TIMESTAMPTZ | NOT NULL DEFAULT now()    | 创建时间                                                             |
| created_by  | UUID        | NOT NULL                  | 创建人                                                               |
| updated_at  | TIMESTAMPTZ | NOT NULL DEFAULT now()    | 更新时间                                                             |
| updated_by  | UUID        | NOT NULL                  | 更新人                                                               |
| is_deleted  | BOOLEAN     | NOT NULL DEFAULT false    | 软删除                                                               |

---

### task_relations（任务关联）

| 字段           | 类型        | 约束                   | 说明                  |
| -------------- | ----------- | ---------------------- | --------------------- |
| id             | UUID        | PK                     | ID                    |
| parent_task_id | UUID        | FK tasks(id)           | 父任务                |
| child_task_id  | UUID        | FK tasks(id)           | 子任务                |
| relation_type  | TEXT        | NOT NULL               | includes / depends_on |
| created_at     | TIMESTAMPTZ | NOT NULL DEFAULT now() | 创建时间              |

---

### resources（资源表）

| 字段           | 类型        | 约束                   | 说明              |
| -------------- | ----------- | ---------------------- | ----------------- |
| id             | UUID        | PK                     | 资源 ID           |
| file_name      | TEXT        | NOT NULL               | 文件名            |
| file_type      | TEXT        | NOT NULL               | pdf / image / doc |
| storage_url    | TEXT        | NOT NULL               | OSS 地址          |
| extracted_text | TEXT        |                        | AI 识别文本       |
| created_at     | TIMESTAMPTZ | NOT NULL DEFAULT now() | 创建时间          |
| created_by     | UUID        | NOT NULL               | 上传人            |
| updated_at     | TIMESTAMPTZ | NOT NULL DEFAULT now() | 更新时间          |
| updated_by     | UUID        | NOT NULL               | 更新人            |
| is_deleted     | BOOLEAN     | NOT NULL DEFAULT false | 软删除            |

---

### task_resources（任务-资源关联）

| 字段        | 类型        | 约束                   | 说明     |
| ----------- | ----------- | ---------------------- | -------- |
| id          | UUID        | PK                     | ID       |
| task_id     | UUID        | FK tasks(id)           | 任务     |
| resource_id | UUID        | FK resources(id)       | 资源     |
| created_at  | TIMESTAMPTZ | NOT NULL DEFAULT now() | 创建时间 |

---

## 三、Migration 设计与实践

### 1. 什么是 Migration？

Migration 是 **数据库结构的版本控制系统**，用于记录每一次表结构的演进。

---

### 2. 为什么 MVP 阶段就必须使用？

- 表结构一定会变化
- 没 migration 会快速积累技术债
- 多环境（本地 / 测试 / 生产）必须一致

---

### 3. Rust + PostgreSQL 推荐方案

**sqlx-cli**

```bash
cargo install sqlx-cli
sqlx migrate add create_tasks
sqlx migrate run
```

---

### 4. Migration 基本约定

- 一次 migration 只做一件事
- 必须提供 down.sql
- 不手改线上表结构

---

## 四、文档角色说明

本文件同时承担：
1. 数据库设计说明书
2. 团队（你自己）规范基准
3. Migration 编写依据

---

## 五、下一步建议

👉 下一步进入 **Rust 数据模型层（struct + sqlx）**

当你准备好，直接说：  
**「下一步：Rust 数据模型层」**
