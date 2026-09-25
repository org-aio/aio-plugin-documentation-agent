ALTER TABLE "boxun_witness_record" ADD COLUMN IF NOT EXISTS "project_id" TEXT;
ALTER TABLE "boxun_witness_record" ADD COLUMN IF NOT EXISTS "number" TEXT;
ALTER TABLE "boxun_witness_record" ADD COLUMN IF NOT EXISTS "test_piece_number" TEXT;
ALTER TABLE "boxun_witness_record" ADD COLUMN IF NOT EXISTS "sampling_quantity" INTEGER;
ALTER TABLE "boxun_witness_record" ADD COLUMN IF NOT EXISTS "sample_and_group_description" TEXT;
ALTER TABLE "boxun_witness_record" ADD COLUMN IF NOT EXISTS "testing_company_name" TEXT;
ALTER TABLE "boxun_witness_record" ADD COLUMN IF NOT EXISTS "file_name" TEXT;
