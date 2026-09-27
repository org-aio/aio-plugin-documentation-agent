-- 委托单生成施工记录时携带项目，确保全新租户也具备所需列。
ALTER TABLE "boxun_wtsj_concrete_construction_record" ADD COLUMN IF NOT EXISTS "project_id" TEXT;

UPDATE "boxun_wtsj_concrete_construction_record" AS record
SET "project_id" = ledger."project_id"
FROM "boxun_wtsj_block_retention_ledger" AS ledger
WHERE record."project_id" IS NULL AND record."fk_sk_id" = ledger."id"::TEXT;
