-- Your SQL goes here
START TRANSACTION;

CREATE TABLE authors (
    id int NOT NULL AUTO_INCREMENT,
    nickname varchar(255) NOT NULL,
    PRIMARY KEY (id)
);

ALTER TABLE posts ADD author_id INT NULL;
ALTER TABLE posts ADD CONSTRAINT fk_author_id FOREIGN KEY (author_id) REFERENCES authors(id);

COMMIT;