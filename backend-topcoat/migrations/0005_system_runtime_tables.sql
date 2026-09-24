ALTER TABLE "system_users" ADD COLUMN IF NOT EXISTS "external_user_id" VARCHAR(255);
ALTER TABLE "system_users" ADD COLUMN IF NOT EXISTS "external_tenant_id" VARCHAR(255);

CREATE TABLE IF NOT EXISTS "infra_file_config" (
  "create_time" TIMESTAMP NOT NULL,
  "update_time" TIMESTAMP,
  "id" BIGINT NOT NULL,
  "updater" BIGINT,
  "creator" BIGINT,
  "name" VARCHAR(255) NOT NULL,
  "storage" INTEGER,
  "master" BOOLEAN NOT NULL,
  "visible" BOOLEAN NOT NULL,
  "config" JSONB,
  "remark" VARCHAR(255),
  PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "system_login_log" (
  "create_time" TIMESTAMP NOT NULL,
  "id" BIGINT NOT NULL,
  "log_type" INTEGER,
  "trace_id" VARCHAR(255),
  "user_id" BIGINT,
  "user_type" INTEGER,
  "username" VARCHAR(255),
  "result" INTEGER,
  "status" INTEGER,
  "user_ip" VARCHAR(255),
  "user_agent" VARCHAR(255),
  PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "system_operate_log" (
  "create_time" TIMESTAMP NOT NULL,
  "id" BIGINT NOT NULL,
  "trace_id" VARCHAR(255),
  "user_type" INTEGER,
  "user_id" BIGINT,
  "user_name" VARCHAR(255),
  "type" VARCHAR(255),
  "sub_type" VARCHAR(255),
  "biz_id" BIGINT,
  "action" VARCHAR(255),
  "extra" TEXT,
  "request_method" VARCHAR(255),
  "request_url" VARCHAR(255),
  "user_ip" VARCHAR(255),
  "user_agent" VARCHAR(255),
  "creator" VARCHAR(255),
  "creator_name" VARCHAR(255),
  PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "infra_api_access_log" (
  "create_time" TIMESTAMP NOT NULL,
  "id" BIGINT NOT NULL,
  "trace_id" VARCHAR(255),
  "user_id" BIGINT,
  "user_type" INTEGER,
  "application_name" VARCHAR(255),
  "request_method" VARCHAR(255),
  "request_params" TEXT,
  "response_body" TEXT,
  "request_url" VARCHAR(255),
  "user_ip" VARCHAR(255),
  "user_agent" VARCHAR(255),
  "operate_module" VARCHAR(255),
  "operate_name" VARCHAR(255),
  "operate_type" INTEGER,
  "begin_time" TIMESTAMP,
  "end_time" TIMESTAMP,
  "duration" BIGINT,
  "result_code" INTEGER,
  "result_msg" VARCHAR(255),
  PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "infra_api_error_log" (
  "create_time" TIMESTAMP NOT NULL,
  "id" BIGINT NOT NULL,
  "trace_id" VARCHAR(255),
  "user_id" BIGINT,
  "user_type" INTEGER,
  "application_name" VARCHAR(255),
  "request_method" VARCHAR(255),
  "request_params" TEXT,
  "request_url" VARCHAR(255),
  "user_ip" VARCHAR(255),
  "user_agent" VARCHAR(255),
  "exception_time" TIMESTAMP,
  "exception_name" VARCHAR(255),
  "exception_message" TEXT,
  "exception_root_cause_message" TEXT,
  "exception_stack_trace" TEXT,
  "exception_class_name" VARCHAR(255),
  "exception_file_name" VARCHAR(255),
  "exception_method_name" VARCHAR(255),
  "exception_line_number" INTEGER,
  "process_user_id" BIGINT,
  "process_status" INTEGER,
  "process_time" TIMESTAMP,
  "result_code" INTEGER,
  PRIMARY KEY ("id")
);
CREATE UNIQUE INDEX IF NOT EXISTS "system_users_external_identity_idx" ON "system_users" ("external_tenant_id", "external_user_id") WHERE "external_user_id" IS NOT NULL;
