# Overview

This docker-compose example is for showing how to take data from an OLTP system (MySQL) and load it into a columnar database (MariaDB ColumnStore and/or Clickhouse). There isn't anything revolutionary with this repo, it's just here to prove that you can connect between MySQL and MariaDB CS or Clickhouse via Kafka. And that MariaDB ColumnStore works with Metabase for SQL queries.

# metabase

The metabase Dockerfile is for manually building the metabase container. I needed this to run metabase docker on an Apple Silicon machine.

You may need to manually curl the `metabase.jar` file and place it in the `metabase/` directory in order to build it.

# mcs1 (data warehouse)

This is MariaDB ColumnStore.

You need to leave `mcs1` because the docker script inside depends on it.

After the container starts, you need to run:

```
docker exec -it mcs1 provision mcs1
```

| Key      | Value        |
| -------- | ------------ |
| UserName | admin        |
| Password | C0lumnStore! |

# insert_test_data

This is a simple Rust console application that just generate a bunch of data (test posts) to insert into MySQL.

# DBZ Config 

This is for creating the MySQL connector

mysql-test
```
{
    "connector.class": "io.debezium.connector.mysql.MySqlConnector",
    "database.user": "root",
    "topic.prefix": "dbz",
    "schema.history.internal.kafka.topic": "schema-changes.test",
    "database.server.id": "1",
    "tasks.max": "1",
    "database.hostname": "mysql",
    "database.password": "password",
    "database.allowPublicKeyRetrieval": "true",
    "database.history.kafka.bootstrap.servers": "kafka:9092",
    "database.history.kafka.topic": "test-history",
    "database.server.name": "test",
    "name": "mysql-test",
    "schema.history.internal.kafka.bootstrap.servers": "kafka:9092",
    "database.port": "3306",
    "database.include.list": "test",
    "key.converter.schema.registry.url": "http://schema-registry:8081",
    "value.converter.schema.registry.url": "http://schema-registry:8081",
    "key.converter": "io.confluent.connect.avro.AvroConverter",
    "value.converter": "io.confluent.connect.avro.AvroConverter"
}
```

This is for creating the JDBC connector to the Operations and Analyics databases.

operations-posts
```
{
    "connector.class": "io.confluent.connect.jdbc.JdbcSinkConnector",
    "connection.url": "jdbc:mariadb://mcs1:3306/test",
    "connection.user": "admin",
    "connection.password": "C0lumnStore!",
    "topic.prefix": "mcs1-operations-",
    "poll.interval.ms" : 3600000,
    "topics": "test.test.posts",
    "name": "operations-posts",
    "insert.mode": "upsert",
    "pk.fields": "post_id",
    "transforms.unwrap.type": "io.debezium.transforms.ExtractNewRecordState",
    "pk.mode": "record_key",
    "transforms": "rename, unwrap, convertTS",
    "transforms.convertTS.type": "org.apache.kafka.connect.transforms.TimestampConverter$Value",
    "transforms.convertTS.field": "post_ts",
    "transforms.convertTS.target.type": "Timestamp",
    "schema.evolution": "none",
    "transforms.rename.type": "org.apache.kafka.connect.transforms.RegexRouter",
    "transforms.rename.regex": ".*\\.(.*)",
    "transforms.rename.replacement": "$1",
    "key.converter": "io.confluent.connect.avro.AvroConverter",
    "key.converter.schema.registry.url": "http://schema-registry:8081",
    "value.converter": "io.confluent.connect.avro.AvroConverter",
    "value.converter.schema.registry.url": "http://schema-registry:8081"
}
```

analytics-posts
```
{
    "connector.class": "io.confluent.connect.jdbc.JdbcSinkConnector",
    "connection.url": "jdbc:mariadb://mcs1:3306/analytics",
    "connection.user": "admin",
    "connection.password": "C0lumnStore!",
    "topic.prefix": "mcs1-operations-",
    "poll.interval.ms" : 3600000,
    "topics": "test.test.posts",
    "name": "analytics-posts",
    "insert.mode": "upsert",
    "pk.fields": "post_id",
    "transforms.unwrap.type": "io.debezium.transforms.ExtractNewRecordState",
    "pk.mode": "record_key",
    "transforms": "rename, unwrap, convertTS",
    "transforms.convertTS.type": "org.apache.kafka.connect.transforms.TimestampConverter$Value",
    "transforms.convertTS.field": "post_ts",
    "transforms.convertTS.target.type": "Timestamp",
    "schema.evolution": "none",
    "transforms.rename.type": "org.apache.kafka.connect.transforms.RegexRouter",
    "transforms.rename.regex": ".*\\.(.*)",
    "transforms.rename.replacement": "$1",
    "key.converter": "io.confluent.connect.avro.AvroConverter",
    "key.converter.schema.registry.url": "http://schema-registry:8081",
    "value.converter": "io.confluent.connect.avro.AvroConverter",
    "value.converter.schema.registry.url": "http://schema-registry:8081"
}
```

This is for connecting Clickhouse to Kafka as a sink for the `posts` topic.

clickhouse-posts

```
{
    "connector.class": "com.clickhouse.kafka.connect.ClickHouseSinkConnector",
    "tasks.max": "1",
    "name": "clickhouse-posts",
    "topics": "test.test.posts",
    "ssl": "false",
    "hostname": "clickhouse",
    "database": "test",
    "password": "",
    "port": "8123",
    "value.converter.schemas.enable": "true",
    "value.converter": "io.confluent.connect.avro.AvroConverter",
    "value.converter.schema.registry.url": "http://schema-registry:8081",
    "key.converter.schema.registry.url": "http://schema-registry:8081",
    "key.converter.schemas.enable": "true",
    "key.converter": "io.confluent.connect.avro.AvroConverter",
    "exactlyOnce": "true",
    "username": "default",
    "schemas.enable": "false",
    "topic2TableMap": "test.test.posts=posts",
    "transforms": "unwrap, convertTS",
    "transforms.unwrap.type": "io.debezium.transforms.ExtractNewRecordState",
    "transforms.convertTS.type": "org.apache.kafka.connect.transforms.TimestampConverter$Value",
    "transforms.convertTS.field": "post_ts",
    "transforms.convertTS.target.type": "Timestamp"
}
```

To validate:

```
 curl -X PUT http://localhost:8083/connector-plugins/com.clickhouse.kafka.connect.ClickHouseSinkConnector/config/validate -H "Content-Type: application/json" -d '{
    "connector.class": "com.clickhouse.kafka.connect.ClickHouseSinkConnector",
    "tasks.max": "1",
    "name": "clickhouse-posts",
    "topics": "test.test.posts",
    "ssl": "false",
    "hostname": "clickhouse",
    "database": "test",
    "password": "",
    "port": "8123",
    "value.converter.schemas.enable": "true",
    "value.converter": "io.confluent.connect.avro.AvroConverter",
    "value.converter.schema.registry.url": "http://schema-registry:8081",
    "key.converter.schema.registry.url": "http://schema-registry:8081",
    "key.converter.schemas.enable": "true",
    "key.converter": "io.confluent.connect.avro.AvroConverter",
    "exactlyOnce": "true",
    "username": "default",
    "schemas.enable": "false",
    "topic2TableMap": "test.test.posts=posts",
    "transforms": "unwrap, convertTS",
    "transforms.unwrap.type": "io.debezium.transforms.ExtractNewRecordState",
    "transforms.convertTS.type": "org.apache.kafka.connect.transforms.TimestampConverter$Value",
    "transforms.convertTS.field": "post_ts",
    "transforms.convertTS.target.type": "Timestamp"
}';
```

# Observations

MariaDB ColumnStore is not optimized for writes, so it lags behind when there is a ton of data to insert. It is preferable to do INSERTs in batches. This becomes obvious over time if you look in Kafka UI for the consumer groups the lag for the analytics database.

In the `init.sql` file, there is another database named `dw` that you can create. To load data into that database, you can simply write an INSERT INTO SELECT statement; as in:

```
INSERT INTO dw.posts SELECT * FROM test.posts;
```

which basically is loading data from your operational data store into your data warehouse.

You may also want to look at the MariaDB ColumnStore documentation for importing data:

- https://mariadb.com/kb/en/columnstore-bulk-data-loading/
- https://mariadb.com/kb/en/columnstore-batch-insert-mode/

Where you can set `set infinidb_use_import_for_batchinsert = (0|1)` for batch insertions.

On my local machine, loading this way directly from the ODS (the `test` database in the `mcs` container) to DW was much faster than waiting for Kafka to keep the `analytics` database up to date with the source `test` database.

Running a query such as:

```
select
	poster_id,
	year(post_ts) post_year,
	count(*) num_posts
from 
	test.posts
group by 
	poster_id,
	year(post_ts);
```

versus 

```
select
	poster_id,
	year(post_ts) post_year,
	count(*) num_posts
from 
	dw.posts
group by 
	poster_id,
	year(post_ts);
```

was also faster.

Using Clickhouse data seemed to load much faster.