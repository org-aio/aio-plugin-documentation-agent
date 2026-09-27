ALTER TABLE "boxun_mobilization_of_raw_materials" ADD COLUMN IF NOT EXISTS "sampling_and_inspection_status" TEXT;

ALTER TABLE "boxun_wtsj_commission_order" ADD COLUMN IF NOT EXISTS "fk_ycl_id" TEXT;
ALTER TABLE "boxun_wtsj_commission_order" ADD COLUMN IF NOT EXISTS "wt_remarks" TEXT;

ALTER TABLE "boxun_wtsj_commission_order_sample" ADD COLUMN IF NOT EXISTS "fk_wt_id" TEXT;
ALTER TABLE "boxun_wtsj_commission_order_sample" ADD COLUMN IF NOT EXISTS "strength_grade" TEXT;
ALTER TABLE "boxun_wtsj_commission_order_sample" ADD COLUMN IF NOT EXISTS "engineering_location" TEXT;
ALTER TABLE "boxun_wtsj_commission_order_sample" ADD COLUMN IF NOT EXISTS "forming_date" DATE;
ALTER TABLE "boxun_wtsj_commission_order_sample" ADD COLUMN IF NOT EXISTS "furnace_batch_number" TEXT;
ALTER TABLE "boxun_wtsj_commission_order_sample" ADD COLUMN IF NOT EXISTS "manufacturer" TEXT;
ALTER TABLE "boxun_wtsj_commission_order_sample" ADD COLUMN IF NOT EXISTS "sample_quantity" TEXT;
ALTER TABLE "boxun_wtsj_commission_order_sample" ADD COLUMN IF NOT EXISTS "representative_batch" TEXT;

UPDATE "boxun_wtsj_commission_order_sample"
SET "fk_wt_id" = "order_id"
WHERE "fk_wt_id" IS NULL AND "order_id" IS NOT NULL;
