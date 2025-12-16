# learnnest —— MVP 0.1 数据库字段说明表 & Migration 指南（v3）

> 版本：v3.1
> 变更：user → account；新增 username/nickname；删除 plan/plan_task 表；task_relation → task_link
> 技术栈：Rust + Salvo + PostgreSQL + Docker
> 目标：为 MVP 0.1 提供**可直接写 migration**的数据库契约

---

## 1. 设计原则

### 1.1 核心理念：Task 统一模型

Task 是系统最核心的实体，统一承载：
- 作业（homework）
- 练习（practice）
- 预习（preview）
- 复习（review）
- 计划（plan）

Task 可以自由组合、嵌套，通过 `task_link` 表表达层级和关联关系。

### 1.2 技术约定

| 项目 | 约定 |
|------|------|
| 主键 | UUID（应用层生成） |
| 表名 | 单数（user, task, resource） |
| 时间 | TIMESTAMPTZ |
| 软删除 | is_deleted BOOLEAN NOT NULL DEFAULT false |
| 枚举 | TEXT + CHECK 约束 |
| 外键 | 不使用数据库外键，应用层保证一致性 |

---

## 2. ER 结构（MVP 0.1）

```
account 1 ──< task
account 1 ──< resource

task N ──< task_link >── N task      （自关联：包含/拆分/依赖）
task N ──< task_resource >── N resource
```

---

## 3. 字段说明表

---

### 3.1 account（账户）

> 注意：不使用 `user` 作为表名，因为在 PostgreSQL 中是保留字

| 字段 | 类型 | 约束 | 说明 |
|------|------|------|------|
| id | UUID | PK | 账户 ID（应用层生成） |
| username | TEXT | UNIQUE | 登录名（预留，MVP 可为空） |
| nickname | TEXT | NOT NULL | 显示名/昵称 |
| role | TEXT | NOT NULL DEFAULT 'parent' | parent / child |
| created_at | TIMESTAMPTZ | NOT NULL DEFAULT now() | 创建时间 |
| created_by | UUID | NOT NULL | 创建人（首次可等于 id） |
| updated_at | TIMESTAMPTZ | NOT NULL DEFAULT now() | 更新时间 |
| updated_by | UUID | NOT NULL | 更新人 |
| is_deleted | BOOLEAN | NOT NULL DEFAULT false | 软删除 |

**CHECK 约束：**
```sql
CHECK (role IN ('parent', 'child'))
```

**索引：**
```sql
CREATE UNIQUE INDEX idx_account_username ON account(username) WHERE username IS NOT NULL;
```

---

### 3.2 task（任务）

> Task 统一承载：作业 / 练习 / 预习 / 复习 / 计划

| 字段 | 类型 | 约束 | 说明 |
|------|------|------|------|
| id | UUID | PK | 任务 ID |
| account_id | UUID | NOT NULL | 归属账户（实现用户隔离） |
| title | TEXT | NOT NULL | 标题 |
| description | TEXT | | 描述 |
| type | TEXT | NOT NULL | homework / practice / preview / review / plan |
| subject | TEXT | NOT NULL | math / chinese / english / physics / chemistry / programming / other |
| status | TEXT | NOT NULL DEFAULT 'active' | active / done / archived |
| due_date | TIMESTAMPTZ | | 截止时间 |
| completed_at | TIMESTAMPTZ | | 完成时间 |
| source | TEXT | NOT NULL DEFAULT 'manual' | manual / ai_extract / wechat / dingtalk |
| created_at | TIMESTAMPTZ | NOT NULL DEFAULT now() | 创建时间 |
| created_by | UUID | NOT NULL | 创建人 |
| updated_at | TIMESTAMPTZ | NOT NULL DEFAULT now() | 更新时间 |
| updated_by | UUID | NOT NULL | 更新人 |
| is_deleted | BOOLEAN | NOT NULL DEFAULT false | 软删除 |

**CHECK 约束：**
```sql
CHECK (type IN ('homework', 'practice', 'preview', 'review', 'plan'))
CHECK (subject IN ('math', 'chinese', 'english', 'physics', 'chemistry', 'programming', 'other'))
CHECK (status IN ('active', 'done', 'archived'))
CHECK (source IN ('manual', 'ai_extract', 'wechat', 'dingtalk'))
```

**索引：**
```sql
CREATE INDEX idx_task_account_id ON task(account_id);
CREATE INDEX idx_task_account_status ON task(account_id, status);
CREATE INDEX idx_task_account_due_date ON task(account_id, due_date);
CREATE INDEX idx_task_account_type ON task(account_id, type);
```

---

### 3.3 task_link（任务关联）

> 用于表示 Task 之间的组合、拆分、依赖关系

| 字段 | 类型 | 约束 | 说明 |
|------|------|------|------|
| parent_task_id | UUID | NOT NULL | 父任务 |
| child_task_id | UUID | NOT NULL | 子任务 |
| link_type | TEXT | NOT NULL | includes / ai_split / depends_on |
| created_at | TIMESTAMPTZ | NOT NULL DEFAULT now() | 创建时间 |
| created_by | UUID | NOT NULL | 创建人 |

**主键：**
```sql
PRIMARY KEY (parent_task_id, child_task_id, link_type)
```

**CHECK 约束：**
```sql
CHECK (link_type IN ('includes', 'ai_split', 'depends_on'))
CHECK (parent_task_id != child_task_id)  -- 防止自引用
```

**索引：**
```sql
CREATE INDEX idx_task_link_parent ON task_link(parent_task_id);
CREATE INDEX idx_task_link_child ON task_link(child_task_id);
```

**link_type 说明：**

| 值 | 含义 | 场景 |
|----|------|------|
| includes | 包含 | 语文作业 包含 字词听写、作文、阅读 |
| ai_split | AI 拆分 | 上传作业截图后，AI 自动拆分出的子任务 |
| depends_on | 依赖（预留） | 任务 B 必须在任务 A 完成后才能开始 |

---

### 3.4 resource（资源）

| 字段 | 类型 | 约束 | 说明 |
|------|------|------|------|
| id | UUID | PK | 资源 ID |
| account_id | UUID | NOT NULL | 归属账户 |
| file_name | TEXT | NOT NULL | 原始文件名 |
| file_type | TEXT | NOT NULL | pdf / image / doc / other |
| storage_url | TEXT | NOT NULL | OSS 地址 |
| extracted_text | TEXT | | AI 提取的文本 |
| created_at | TIMESTAMPTZ | NOT NULL DEFAULT now() | 创建时间 |
| created_by | UUID | NOT NULL | 上传人 |
| updated_at | TIMESTAMPTZ | NOT NULL DEFAULT now() | 更新时间 |
| updated_by | UUID | NOT NULL | 更新人 |
| is_deleted | BOOLEAN | NOT NULL DEFAULT false | 软删除 |

**CHECK 约束：**
```sql
CHECK (file_type IN ('pdf', 'image', 'doc', 'other'))
```

**索引：**
```sql
CREATE INDEX idx_resource_account_id ON resource(account_id);
CREATE INDEX idx_resource_account_created ON resource(account_id, created_at);
```

---

### 3.5 task_resource（任务-资源关联）

| 字段 | 类型 | 约束 | 说明 |
|------|------|------|------|
| task_id | UUID | NOT NULL | 任务 |
| resource_id | UUID | NOT NULL | 资源 |
| created_at | TIMESTAMPTZ | NOT NULL DEFAULT now() | 创建时间 |
| created_by | UUID | NOT NULL | 创建人 |

**主键：**
```sql
PRIMARY KEY (task_id, resource_id)
```

**索引：**
```sql
CREATE INDEX idx_task_resource_resource ON task_resource(resource_id);
```

---

## 4. Migration 指南

### 4.1 推荐工具：sqlx-cli

```bash
cargo install sqlx-cli
sqlx migrate add 0001_init
sqlx migrate run
sqlx migrate revert
```

### 4.2 目录结构

```
backend/
  migrations/
    0001_init.sql          # 建表 + CHECK + 主键
    0002_indexes.sql       # 索引（可选，也可合并到 0001）
```

### 4.3 Migration 约定

- 一次 migration 只做一类变更
- 建表时就加 CHECK 约束和主键
- 先建被依赖的表（account → task → task_link）

---

## 5. 示例数据

```sql
-- 账户
INSERT INTO account (id, username, nickname, role, created_by, updated_by)
VALUES ('550e8400-e29b-41d4-a716-446655440001', NULL, '小明', 'child',
        '550e8400-e29b-41d4-a716-446655440001',
        '550e8400-e29b-41d4-a716-446655440001');

-- 语文作业（父任务）
INSERT INTO task (id, account_id, title, type, subject, created_by, updated_by)
VALUES ('550e8400-e29b-41d4-a716-446655440010',
        '550e8400-e29b-41d4-a716-446655440001',
        '12/16 语文作业', 'homework', 'chinese',
        '550e8400-e29b-41d4-a716-446655440001',
        '550e8400-e29b-41d4-a716-446655440001');

-- 子任务：字词听写
INSERT INTO task (id, account_id, title, type, subject, created_by, updated_by)
VALUES ('550e8400-e29b-41d4-a716-446655440011',
        '550e8400-e29b-41d4-a716-446655440001',
        '字词听写', 'homework', 'chinese',
        '550e8400-e29b-41d4-a716-446655440001',
        '550e8400-e29b-41d4-a716-446655440001');

-- 关联
INSERT INTO task_link (parent_task_id, child_task_id, link_type, created_by)
VALUES ('550e8400-e29b-41d4-a716-446655440010',
        '550e8400-e29b-41d4-a716-446655440011',
        'includes',
        '550e8400-e29b-41d4-a716-446655440001');
```

---

## 6. 版本历史

| 版本 | 日期 | 变更 |
|------|------|------|
| v1 | - | 初版 |
| v2 | - | 补齐 user_id、补齐 Plan/PlanTask |
| v3 | 2024/12/16 | 删除 plan/plan_task 表；task_relation → task_link；调整枚举值 |
| v3.1 | 2024/12/16 | user → account；新增 username/nickname 字段 |

---

## 7. 下一步

1. 编写 `0001_init.sql` migration
2. Rust 数据模型（SQLx struct）
3. API 设计
