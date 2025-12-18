-- 添加密码哈希字段
-- 创建时间: 2024-12-18

ALTER TABLE account ADD COLUMN password_hash TEXT;
