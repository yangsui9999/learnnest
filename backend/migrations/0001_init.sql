-- learnnest MVP 0.1 初始化 migration
-- 创建时间: 2024-12-16
-- 说明: 创建核心表结构（account, task, task_link, resource, task_resource）

-- ============================================
-- 1. account 表（账户）
-- ============================================
CREATE TABLE account (
    id UUID PRIMARY KEY,
    username TEXT UNIQUE,                              -- 登录名（预留，可为空）
    nickname TEXT NOT NULL,                            -- 显示名/昵称
    role TEXT NOT NULL DEFAULT 'parent',               -- parent / child
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    created_by UUID NOT NULL,                          -- 创建人（首次可等于 id）
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_by UUID NOT NULL,
    is_deleted BOOLEAN NOT NULL DEFAULT false,

    CONSTRAINT chk_account_role CHECK (role IN ('parent', 'child'))
);

-- account 索引
CREATE UNIQUE INDEX idx_account_username ON account(username) WHERE username IS NOT NULL;

-- ============================================
-- 2. task 表（任务）
-- ============================================
-- Task 统一承载：作业 / 练习 / 预习 / 复习 / 计划
CREATE TABLE task (
    id UUID PRIMARY KEY,
    account_id UUID NOT NULL,                          -- 归属账户（实现用户隔离）
    title TEXT NOT NULL,
    description TEXT,
    type TEXT NOT NULL,                                -- homework / practice / preview / review / plan
    subject TEXT NOT NULL,                             -- math / chinese / english / physics / chemistry / programming / other
    status TEXT NOT NULL DEFAULT 'active',             -- active / done / archived
    due_date TIMESTAMPTZ,                              -- 截止时间
    completed_at TIMESTAMPTZ,                          -- 完成时间
    source TEXT NOT NULL DEFAULT 'manual',             -- manual / ai_extract / wechat / dingtalk
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    created_by UUID NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_by UUID NOT NULL,
    is_deleted BOOLEAN NOT NULL DEFAULT false,

    CONSTRAINT chk_task_type CHECK (type IN ('homework', 'practice', 'preview', 'review', 'plan')),
    CONSTRAINT chk_task_subject CHECK (subject IN ('math', 'chinese', 'english', 'physics', 'chemistry', 'programming', 'other')),
    CONSTRAINT chk_task_status CHECK (status IN ('active', 'done', 'archived')),
    CONSTRAINT chk_task_source CHECK (source IN ('manual', 'ai_extract', 'wechat', 'dingtalk'))
);

-- task 索引
CREATE INDEX idx_task_account_id ON task(account_id);
CREATE INDEX idx_task_account_status ON task(account_id, status);
CREATE INDEX idx_task_account_due_date ON task(account_id, due_date);
CREATE INDEX idx_task_account_type ON task(account_id, type);

-- ============================================
-- 3. task_link 表（任务关联）
-- ============================================
-- 用于表示 Task 之间的组合、拆分、依赖关系
CREATE TABLE task_link (
    parent_task_id UUID NOT NULL,                      -- 父任务
    child_task_id UUID NOT NULL,                       -- 子任务
    link_type TEXT NOT NULL,                           -- includes / ai_split / depends_on
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    created_by UUID NOT NULL,

    PRIMARY KEY (parent_task_id, child_task_id, link_type),

    CONSTRAINT chk_task_link_type CHECK (link_type IN ('includes', 'ai_split', 'depends_on')),
    CONSTRAINT chk_task_link_no_self_ref CHECK (parent_task_id != child_task_id)
);

-- task_link 索引
CREATE INDEX idx_task_link_parent ON task_link(parent_task_id);
CREATE INDEX idx_task_link_child ON task_link(child_task_id);

-- ============================================
-- 4. resource 表（资源）
-- ============================================
CREATE TABLE resource (
    id UUID PRIMARY KEY,
    account_id UUID NOT NULL,                          -- 归属账户
    file_name TEXT NOT NULL,                           -- 原始文件名
    file_type TEXT NOT NULL,                           -- pdf / image / doc / other
    storage_url TEXT NOT NULL,                         -- OSS 地址
    extracted_text TEXT,                               -- AI 提取的文本
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    created_by UUID NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_by UUID NOT NULL,
    is_deleted BOOLEAN NOT NULL DEFAULT false,

    CONSTRAINT chk_resource_file_type CHECK (file_type IN ('pdf', 'image', 'doc', 'other'))
);

-- resource 索引
CREATE INDEX idx_resource_account_id ON resource(account_id);
CREATE INDEX idx_resource_account_created ON resource(account_id, created_at);

-- ============================================
-- 5. task_resource 表（任务-资源关联）
-- ============================================
CREATE TABLE task_resource (
    task_id UUID NOT NULL,
    resource_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    created_by UUID NOT NULL,

    PRIMARY KEY (task_id, resource_id)
);

-- task_resource 索引
CREATE INDEX idx_task_resource_resource ON task_resource(resource_id);

-- ============================================
-- 完成
-- ============================================
-- 表清单：account, task, task_link, resource, task_resource
-- 注意：不使用数据库外键，应用层保证数据一致性
