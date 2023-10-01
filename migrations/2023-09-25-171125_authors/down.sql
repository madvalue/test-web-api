-- This file should undo anything in `up.sql`
START TRANSACTION;

ALTER TABLE posts DROP FOREIGN KEY fk_author_id;
ALTER TABLE posts DROP COLUMN author_id;

DROP TABLE authors;

COMMIT;