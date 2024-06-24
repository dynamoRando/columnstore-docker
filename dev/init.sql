-- This script is for the `mysql` container; it's supposed to represent your OLTP database
CREATE DATABASE IF NOT EXISTS test;

USE test;

CREATE TABLE
    IF NOT EXISTS posters (id INT, PRIMARY KEY (id));

INSERT INTO
    posters (id)
VALUES
    (1)
  , (2)
  , (3);

-- we'll have a script generator load data into this table once Kafka DBZ -> JDBC to mcs1 is configured
CREATE TABLE
    IF NOT EXISTS posts (
        post_id INT
      , poster_id INT
      , post_ts datetime
      , PRIMARY KEY (post_id)
    );

INSERT INTO
    posts (post_id, poster_id, post_ts)
VALUES
    (0, 0, '1970-01-01');

-- This portion of the script is for the MariaDB CS container. We'll have two databases in it, a "test" database serving as an Operational Data Store, and 
-- the "analytics" database, which will be based on the CS engine, for analytical queries.
CREATE DATABASE test;

CREATE TABLE
    IF NOT EXISTS test.posts (
        post_id INT
      , poster_id INT
      , post_ts datetime
      , PRIMARY KEY (post_id)
    ) ENGINE = InnoDB;

CREATE DATABASE analytics;

CREATE TABLE
    IF NOT EXISTS analytics.posts (post_id INT, poster_id INT, post_ts datetime) ENGINE = ColumnStore;

-- this database is for bulk loading, rather than trying to move data via Kafka

CREATE DATABASE dw;

CREATE TABLE
    IF NOT EXISTS dw.posts (post_id INT, poster_id INT, post_ts datetime) ENGINE = ColumnStore;

-- This is for ClickHouse

CREATE DATABASE test;

CREATE TABLE test.posts
(
    post_id Int32,
    poster_id Int32,
    post_ts DateTime
)
ENGINE = MergeTree
ORDER BY post_id;