-- Add down migration script here
DROP TRIGGER IF EXISTS book_updated_at_trigger ON books;
DROP TABLE IF EXISTS books;

DROP FUNCTION IF EXISTS set_updated_at;