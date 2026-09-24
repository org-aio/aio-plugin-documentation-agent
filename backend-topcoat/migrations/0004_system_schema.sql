CREATE TABLE IF NOT EXISTS "system_users" (
  "create_time" TIMESTAMP NOT NULL,
  "update_time" TIMESTAMP,
  "id" BIGINT NOT NULL,
  "updater" BIGINT,
  "creator" BIGINT,
  "tenant_id" BIGINT,
  "remark" VARCHAR(255),
  "status" INTEGER NOT NULL,
  "username" VARCHAR(255) NOT NULL,
  "password" VARCHAR(255),
  "nickname" VARCHAR(255),
  "dept_id" BIGINT,
  "email" VARCHAR(255),
  "mobile" VARCHAR(255),
  "sex" INTEGER,
  "avatar" VARCHAR(255),
  "login_ip" VARCHAR(255),
  "login_date" TIMESTAMP,
  "realname" VARCHAR(255),
  "current_area_id" BIGINT,
  PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "system_role" (
  "create_time" TIMESTAMP NOT NULL,
  "update_time" TIMESTAMP,
  "id" BIGINT NOT NULL,
  "updater" BIGINT,
  "creator" BIGINT,
  "code" VARCHAR(255) NOT NULL,
  "sort" INTEGER,
  "name" VARCHAR(255),
  "tenant_id" BIGINT,
  "remark" VARCHAR(255),
  "status" INTEGER NOT NULL,
  "type" INTEGER,
  "data_scope" INTEGER,
  "data_scope_dept_ids" JSONB,
  PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "system_menu" (
  "create_time" TIMESTAMP NOT NULL,
  "update_time" TIMESTAMP,
  "id" BIGINT NOT NULL,
  "updater" BIGINT,
  "creator" BIGINT,
  "name" VARCHAR(255) NOT NULL,
  "permission" VARCHAR(255),
  "type" INTEGER NOT NULL,
  "sort" INTEGER NOT NULL,
  "parent_id" BIGINT,
  "path" VARCHAR(255),
  "icon" VARCHAR(255),
  "component" VARCHAR(255),
  "component_name" VARCHAR(255),
  "status" INTEGER NOT NULL,
  "visible" BOOLEAN,
  "keep_alive" BOOLEAN,
  "always_show" BOOLEAN,
  PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "system_dept" (
  "create_time" TIMESTAMP NOT NULL,
  "update_time" TIMESTAMP,
  "id" BIGINT NOT NULL,
  "updater" BIGINT,
  "creator" BIGINT,
  "name" VARCHAR(255),
  "tenant_id" BIGINT,
  "sort" INTEGER,
  "parent_id" BIGINT,
  "status" INTEGER NOT NULL,
  "leader_user_id" BIGINT,
  "phone" VARCHAR(255),
  "email" VARCHAR(255),
  PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "system_post" (
  "create_time" TIMESTAMP NOT NULL,
  "update_time" TIMESTAMP,
  "id" BIGINT NOT NULL,
  "updater" BIGINT,
  "creator" BIGINT,
  "code" VARCHAR(255) NOT NULL,
  "name" VARCHAR(255) NOT NULL,
  "sort" INTEGER,
  "remark" VARCHAR(255),
  "status" INTEGER NOT NULL,
  PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "system_user_role" (
  "user_id" BIGINT NOT NULL,
  "role_id" BIGINT NOT NULL,
  PRIMARY KEY ("user_id", "role_id")
);

CREATE TABLE IF NOT EXISTS "system_user_post" (
  "user_id" BIGINT NOT NULL,
  "post_id" BIGINT NOT NULL,
  PRIMARY KEY ("user_id", "post_id")
);

CREATE TABLE IF NOT EXISTS "system_user_ext" (
  "id" BIGINT NOT NULL,
  "user_id" BIGINT NOT NULL,
  "create_time" TIMESTAMP NOT NULL,
  "update_time" TIMESTAMP,
  "updater" BIGINT,
  "creator" BIGINT,
  "signature" VARCHAR(255),
  "introduction" VARCHAR(255),
  PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "system_dict_type" (
  "create_time" TIMESTAMP NOT NULL,
  "update_time" TIMESTAMP,
  "id" BIGINT NOT NULL,
  "updater" BIGINT,
  "creator" BIGINT,
  "remark" VARCHAR(255),
  "name" VARCHAR(255) NOT NULL,
  "type" VARCHAR(255) NOT NULL,
  "status" INTEGER NOT NULL,
  "deleted_time" TIMESTAMP,
  "deleted" INTEGER NOT NULL,
  PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "system_dict_data" (
  "create_time" TIMESTAMP NOT NULL,
  "update_time" TIMESTAMP,
  "id" BIGINT NOT NULL,
  "updater" BIGINT,
  "creator" BIGINT,
  "remark" VARCHAR(255),
  "sort" INTEGER,
  "status" INTEGER NOT NULL,
  "deleted" INTEGER NOT NULL,
  "label" VARCHAR(255) NOT NULL,
  "value" VARCHAR(255) NOT NULL,
  "dict_type" VARCHAR(255) NOT NULL,
  "color_type" VARCHAR(255),
  "css_class" VARCHAR(255),
  PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "role_menu_mapping" (
  "role_id" BIGINT NOT NULL,
  "menu_id" BIGINT NOT NULL,
  PRIMARY KEY ("role_id", "menu_id")
);

CREATE TABLE IF NOT EXISTS "infra_config" (
  "create_time" TIMESTAMP NOT NULL,
  "update_time" TIMESTAMP,
  "id" BIGINT NOT NULL,
  "updater" BIGINT,
  "creator" BIGINT,
  "remark" VARCHAR(255),
  "category" VARCHAR(255),
  "type" INTEGER,
  "name" VARCHAR(255),
  "config_key" VARCHAR(255),
  "value" VARCHAR(255),
  "visible" BOOLEAN,
  PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "infra_file" (
  "create_time" TIMESTAMP NOT NULL,
  "update_time" TIMESTAMP,
  "id" BIGINT NOT NULL,
  "updater" BIGINT,
  "creator" BIGINT,
  "name" VARCHAR(255),
  "config_id" BIGINT,
  "path" VARCHAR(255) NOT NULL,
  "url" VARCHAR(255) NOT NULL,
  "type" VARCHAR(255),
  "size" INTEGER NOT NULL,
  PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "infra_file_content" (
  "id" BIGINT NOT NULL,
  "config_id" BIGINT NOT NULL,
  "path" TEXT NOT NULL,
  "content" BYTEA NOT NULL,
  "creator" TEXT,
  "create_time" TIMESTAMP NOT NULL,
  "updater" TEXT,
  "update_time" TIMESTAMP NOT NULL,
  "deleted" SMALLINT NOT NULL,
  PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "system_mail_account" (
  "create_time" TIMESTAMP NOT NULL,
  "update_time" TIMESTAMP,
  "id" BIGINT NOT NULL,
  "updater" BIGINT,
  "creator" BIGINT,
  "mail" VARCHAR(255) NOT NULL,
  "username" VARCHAR(255),
  "password" VARCHAR(255),
  "host" VARCHAR(255) NOT NULL,
  "port" INTEGER NOT NULL,
  "ssl_enable" BOOLEAN,
  "starttls_enable" BOOLEAN,
  PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "system_mail_log" (
  "create_time" TIMESTAMP NOT NULL,
  "update_time" TIMESTAMP,
  "id" BIGINT NOT NULL,
  "updater" BIGINT,
  "creator" BIGINT,
  "user_id" BIGINT,
  "user_type" INTEGER,
  "to_mails" JSONB,
  "cc_mails" JSONB,
  "bcc_mails" JSONB,
  "account_id" BIGINT,
  "from_mail" VARCHAR(255),
  "template_id" BIGINT,
  "template_code" VARCHAR(255),
  "template_nickname" VARCHAR(255),
  "template_title" VARCHAR(255),
  "template_content" VARCHAR(255),
  "template_params" JSONB,
  "send_status" INTEGER,
  "send_time" TIMESTAMP,
  "send_message_id" VARCHAR(255),
  "send_exception" VARCHAR(255),
  PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "system_mail_template" (
  "create_time" TIMESTAMP NOT NULL,
  "update_time" TIMESTAMP,
  "id" BIGINT NOT NULL,
  "updater" BIGINT,
  "creator" BIGINT,
  "remark" VARCHAR(255),
  "status" INTEGER NOT NULL,
  "name" VARCHAR(255) NOT NULL,
  "code" VARCHAR(255) NOT NULL,
  "account_id" BIGINT,
  "nickname" VARCHAR(255),
  "title" VARCHAR(255) NOT NULL,
  "content" VARCHAR(255) NOT NULL,
  "params" JSONB,
  PRIMARY KEY ("id")
);

CREATE INDEX IF NOT EXISTS "system_users_username_idx" ON "system_users" ("username");
CREATE INDEX IF NOT EXISTS "system_menu_parent_sort_idx" ON "system_menu" ("parent_id", "sort");
CREATE INDEX IF NOT EXISTS "system_dict_data_type_idx" ON "system_dict_data" ("dict_type");
CREATE INDEX IF NOT EXISTS "infra_config_key_idx" ON "infra_config" ("config_key");
