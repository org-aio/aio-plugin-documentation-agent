-- 补齐商混表单与关联记录生成使用的字段，旧数据沿用既有项目和厂家值。
ALTER TABLE boxun_wtsj_commercial_concrete_ledger
    ADD COLUMN IF NOT EXISTS fk_project_id TEXT,
    ADD COLUMN IF NOT EXISTS name_of_commercial_mixing_station TEXT,
    ADD COLUMN IF NOT EXISTS impermeability_level TEXT,
    ADD COLUMN IF NOT EXISTS strength_remarks TEXT,
    ADD COLUMN IF NOT EXISTS number_of_standard_curing_specimen_groups INTEGER,
    ADD COLUMN IF NOT EXISTS number_of_sets_of_impermeable_test_pieces INTEGER,
    ADD COLUMN IF NOT EXISTS number_of_specimens_in_the_same_culture INTEGER,
    ADD COLUMN IF NOT EXISTS number_of_demolding_specimen_groups INTEGER,
    ADD COLUMN IF NOT EXISTS syntrophic_temperature DOUBLE PRECISION,
    ADD COLUMN IF NOT EXISTS accompanying_documents TEXT,
    ADD COLUMN IF NOT EXISTS day28_data TEXT;

UPDATE boxun_wtsj_commercial_concrete_ledger
SET fk_project_id = project_id WHERE fk_project_id IS NULL;
UPDATE boxun_wtsj_commercial_concrete_ledger
SET name_of_commercial_mixing_station = manufacturer
WHERE name_of_commercial_mixing_station IS NULL;

ALTER TABLE boxun_bystander_record ADD COLUMN IF NOT EXISTS project_id TEXT;
ALTER TABLE boxun_wtsj_concrete_construction_record
    ADD COLUMN IF NOT EXISTS fk_sh_id TEXT,
    ADD COLUMN IF NOT EXISTS number_of_standard_curing_specimen_groups INTEGER,
    ADD COLUMN IF NOT EXISTS number_of_sets_of_impermeable_test_pieces INTEGER,
    ADD COLUMN IF NOT EXISTS number_of_specimens_in_the_same_culture INTEGER,
    ADD COLUMN IF NOT EXISTS number_of_demolding_specimen_groups INTEGER;
