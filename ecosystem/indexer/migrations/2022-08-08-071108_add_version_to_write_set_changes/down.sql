-- This file should undo anything in `up.sql`
alter table write_set_changes drop column if exists version CASCADE;