# learnnest PRD（产品需求文档）v1

> 生成日期：2025-12-16  
> 适用范围：learnnest 家庭学习管理系统（Web + 移动端浏览器）  
> 本文档目标：把我们迄今讨论的内容**收敛成可执行的产品蓝图**，包含 v0.1 MVP 与未来迭代路线，便于长期按计划推进。

---

## 1. 背景与目标

### 1.1 背景
家庭学习场景中，作业/练习/资料来源多样（老师在微信/钉钉布置、图片截图、PDF/Word 资料、课外班答案等），家长整理成本高，孩子自主回看与复盘不方便。

### 1.2 产品定位
- 面向家庭内部（家长 + 孩子）的**学习任务与资料管理系统**。
- 核心理念：支持学习者自驱（计划、执行、复盘），并为后续统计分析与正反馈激励提供数据基础。

### 1.3 产品目标（短中长期）
- **短期（v0.1）**：能用、可持续记录；任务管理 + 资源池 + 截图/资料导入。
- **中期（v0.2~v0.4）**：AI 识别与拆分、提醒、基础统计。
- **长期（v1.x）**：间隔复习（SRS）、错题/知识点分析、激励体系、移动端体验深化（PWA/可能的 App）。

### 1.4 非目标（v0.1 明确不做）
- 完整的班级/同学协作体系
- 复杂的权限系统（多家庭、多角色细粒度权限）
- 深度的错题知识图谱（后续做）

---

## 2. 用户画像与核心场景

### 2.1 用户画像
- 家长：负责收集作业/资料、安排学习、查看完成情况。
- 孩子：执行任务、查看资料/答案、逐步形成自我规划能力。

### 2.2 典型场景（按优先级）
1. **老师在微信群/钉钉布置作业** → 家长截图 → 上传 → 系统展示/可转文字 → 生成任务 → 孩子勾选完成。
2. **老师分享 PDF/Word 资料** → 上传资源池 → 关联到对应作业/计划 → 孩子做完后打开 PDF 对答案。
3. **自主学习**：家长/孩子制定学习计划（Plan）→ 拆解为任务（Task）→ 日常执行与复盘。
4. **后续增强**：自动复习计划、提醒、统计、激励等。

---

## 3. 信息架构（IA）与页面结构（MVP 到未来）

### 3.1 v0.1 页面（MVP 必须）
- **首页 / 今日**：今日任务 + 未完成任务（移动优先）
- **任务列表**：按状态/学科/类型筛选（基础版即可）
- **任务详情**：任务信息、关联资源、关联子任务/拆分任务（如 AI 拆分）
- **创建/编辑任务**：手动创建任务（类型/学科/截止时间/描述）
- **资源池**：上传、列表、预览/下载、关联到任务
- **截图导入**：上传图片 → 调用 LLM → 生成任务（可一次生成多个 Task）

> 说明：日历视图在 v0.1 不强制实现；先保证“今日/未完成”视角可用。

### 3.2 v0.2~v0.4 可能新增页面（增强）
- 日历视图（周/月）与拖拽（可选）
- 统计面板（周完成率、学科趋势）
- 提醒中心（站内提醒；后续扩展到微信/短信）
- 任务搜索（全局搜索任务与资源）

### 3.3 v1.x 页面（长期）
- 间隔复习（SRS）面板（待复习队列）
- 错题/知识点分析面板
- 激励中心（徽章/积分/目标达成）
- 移动端体验深化（PWA/可能 App）

---

## 4. 功能清单与优先级（Roadmap）

> 说明：以“能跑通”为导向，将功能分层：MVP（必须）/ Next（重要）/ Future（长期）

### 4.1 v0.1 MVP（本期定稿必须）
#### A. 账户与数据隔离
- 用户体系（最小可用）
  - 先支持一种登录方式跑通（开发期可用“本地账号/测试账号”）
  - 数据以 user 维度隔离（所有核心数据均绑定 user_id）

#### B. 任务系统（Task）
- 任务类型（type）：homework / practice / preview / review / plan
- 学科（subject）：math / chinese / english / physics / chemistry / programming / other
- 任务状态（status）：active / done / archived（软删用 is_deleted）
- 任务 CRUD：创建、编辑、标记完成、归档、软删除
- 截止时间（due_date，可选）

#### C. 计划（Plan）——通过 Task 统一承载
- 计划是 Task 的一种类型（type='plan'）
- 计划与子任务通过 task_link 关联（link_type='includes'）
- v0.1 不要求进度条/自动拆解，仅提供组织能力
- 不单独建 plan 表，保持模型简洁

#### D. 资源池（Resource）——必须
- 上传图片/PDF/Word（保存到 OSS）
- 资源列表、预览/下载
- 资源可关联到任务（task_resource 多对多）
- 资源支持 extracted_text 字段预留（存 AI 提取文字）

#### E. 截图导入 → AI（LLM）识别与生成任务
- 上传截图到 OSS
- 后端调用多模态 LLM（GLM/DeepSeek/Qwen/豆包等）
- 产出结构化结果：可生成一个或多个 Task（必要时建立 task_relation: ai_split）

#### F. 审计字段规范（全表统一）
- created_at / created_by / updated_at / updated_by / is_deleted

---

### 4.2 v0.2（Next：重要但可后置）
- 登录增强：微信登录、手机号验证码登录、用户名密码登录
- 站内提醒（截止前 N 小时/天）
- 任务搜索（标题/描述/资源名）
- 简版统计：周完成率、学科任务数/完成数
- 日历视图（可选，若任务量变大再做）

---

### 4.3 v0.3~v0.4（增强学习效果）
- OCR/LLM Pipeline 优化（降低成本、提升准确率）
  - OCR 前置提取文字（可缓存）
  - LLM 负责理解与拆分
  - 结构化拆分：题号/段落/选项
- 分享：作业/计划卡片分享到微信（后续）
- 资源池增强：标签/分类/更强筛选

---

### 4.4 v1.x（长期目标）
#### A. 间隔复习（SRS）
- 基于艾宾浩斯 / SM-2 的复习计划生成
- 完成反馈（困难/一般/掌握）动态调整间隔
- 待复习队列与提醒

#### B. 深度统计与审计
- 学科趋势、计划 vs 实际完成偏差
- 子任务正确率/错误率
- 高频错误知识点、薄弱模块

#### C. 正反馈激励机制
- 积分、徽章、成就、连续打卡
- 目标达成反馈（可配置规则）

#### D. 移动端体验
- PWA（离线、桌面图标、推送）
- 可能的原生 App（视推广情况）

---

## 5. 数据模型（概念模型）

> 目标：支持任务、计划、资源、AI 拆分、未来统计扩展

### 5.1 核心实体
- Account：账户（原 User，避免 PostgreSQL 保留字）
- Task：任务（五种 type，包括 plan）
- Resource：资源（文件）
- TaskLink：任务关联（包含/AI拆分/依赖）
- TaskResource：任务-资源关联

> 注意：Plan 不再是独立表，而是 Task 的一种类型（type='plan'），通过 TaskLink 关联子任务

---

## 6. ER 图（MVP 0.1 定稿）

```
account 1 ──< task
account 1 ──< resource

task N ──< task_link >── N task      （自关联：includes/ai_split/depends_on）
task N ──< task_resource >── N resource
```

> 说明：Plan 通过 task.type='plan' 实现，不单独建表

---

## 7. 数据库字段规范（摘要）

> 详细字段说明以《mvp_0_1_db_schema_and_migration_v3.md》为准（单数表名）。

### 7.1 通用字段（所有业务表）
- created_at TIMESTAMPTZ NOT NULL DEFAULT now()
- created_by UUID NOT NULL
- updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
- updated_by UUID NOT NULL
- is_deleted BOOLEAN NOT NULL DEFAULT false

### 7.2 关键隔离字段
- task.account_id UUID NOT NULL
- resource.account_id UUID NOT NULL

### 7.3 命名变更说明
- user → account（避免 PostgreSQL 保留字）
- user_id → account_id
- task_relation → task_link
- relation_type: includes / ai_split / depends_on
- source: manual / ai_extract / wechat / dingtalk

---

## 8. AI（截图导入）功能规格（MVP 0.1）

### 8.1 输入
- 图片（老师布置作业截图、拍照）
- 由前端上传到 OSS，后端拿到 storage_url

### 8.2 处理（MVP：一步到位）
- 后端调用多模态 LLM
- 期望输出结构（示例）：

```json
{
  "tasks": [
    {
      "title": "数学作业",
      "type": "homework",
      "subject": "math",
      "due_date": null,
      "items": ["第1题", "第2题", "第3题"]
    }
  ]
}
```

### 8.3 产出与落库
- 每个 task 生成一条 Task
- 若由同一截图拆分出多个任务：
  - 可用 task_link 记录 ai_split 关系（parent="截图导入任务"或某聚合任务）

### 8.4 未来优化（Pipeline）
- OCR 前置提取文字 → 文本清洗 → LLM 结构化拆分 → 缓存结果（resource.extracted_text / 独立表）

---

## 9. 技术架构与工程约定（当前讨论结果）

### 9.1 技术选型（已确认）
- 前端：Vue 3（Vite 工具链），移动优先响应式
- 后端：Rust + Salvo
- DB：PostgreSQL
- 文件：OSS
- 部署：Docker（后期可 CI/CD）
- AI：LLM API（多模态）

### 9.2 仓库组织（推荐：单仓库、逻辑分离）
- monorepo：
  - frontend/（Vue）
  - backend/（Rust）
  - deploy/（compose/nginx）
  - docs/

### 9.3 多环境配置（原则）
- 后端：ENV 优先 + 配置文件兜底（dev/prod）
- 前端：建议运行时配置（避免每环境都重新 build）
- deploy：docker-compose base + override（dev/prod）

---

## 10. 非功能需求（NFR）

### 10.1 安全
- OSS 访问建议采用签名 URL 或受控下载（后续）
- API 鉴权（最小可用：token）
- 配置文件与密钥不进 git（提供 .env.example）

### 10.2 性能与资源
- 个人服务器资源有限：Rust 服务保持轻量
- AI 调用要可控：记录调用日志、可重试、可缓存（后续）

### 10.3 可维护性
- migration 管理表结构版本
- 文档为单一事实源（SSOT）

---

## 11. Migration 规范（写进 PRD 的原因）

### 11.1 为什么要写进 PRD
- 数据库是长期资产，结构演进必须可追溯
- 多环境一致性依赖 migration
- Rust 强类型项目改结构成本更高，越早规范越省返工

### 11.2 推荐工具
- sqlx-cli（创建与执行迁移）

### 11.3 基本规则
- 一次 migration 做一件事（建表/加索引/改字段）
- 线上不手改表结构，全部走迁移

---

## 12. 验收标准（v0.1）

- 能创建账户（或使用测试账户）并隔离数据
- 能创建/编辑/完成/归档/软删除任务
- 能创建计划类型任务（type='plan'）并通过 task_link 关联子任务
- 能上传资源到 OSS、预览/下载、关联到任务
- 能上传截图并通过 LLM 生成任务（至少生成 1 个 Task）
- 数据库表结构与 v3 字段说明表一致，迁移可一键跑通

---

## 13. 文档清单（建议作为 docs/ 的索引）

- learnnest_prd_v1.md（本文：全量蓝图）
- mvp_0_1_db_schema_and_migration_v3.md（数据库字段说明 + migration，最新版）
- solo_developer_methodology.md（个人开发者工作方法论）
- （可选）learning_mvp_design.md（历史草稿，可保留但不再作为事实源）

---

## 14. 版本变更记录
- v1：首次整合 PRD（补齐未来 TODO 与路线图；对齐 Plan/资源池/AI 截图导入/移动端/PWA/统计/激励等讨论内容）
- v1.1（2024/12/16）：同步数据库设计变更
  - user → account
  - 删除 plan/plan_task 表，Plan 改为 task.type='plan'
  - task_relation → task_link
  - 更新字段命名和引用
