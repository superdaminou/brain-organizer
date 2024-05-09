ALTER TABLE reference ADD COLUMN date_creation TEXT;
UPDATE reference set date_creation=date('now');