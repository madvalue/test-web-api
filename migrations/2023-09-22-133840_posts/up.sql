-- Your SQL goes here
CREATE TABLE posts (
    id int NOT NULL AUTO_INCREMENT,
    title varchar(255) NOT NULL,
    body text NOT NULL,
    PRIMARY KEY (id)
);