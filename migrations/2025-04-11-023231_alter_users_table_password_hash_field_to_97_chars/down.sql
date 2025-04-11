-- This file should undo anything in `up.sql`
ALTER TABLE "user"
ALTER COLUMN password_hash TYPE VARCHAR(96);
