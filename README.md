# Overview

This docker-compose example is for showing how to take data from an OLTP system (MySQL) and load it into a columnar database (MariaDB ColumnStore). There isn't anything revolutionary with this repo, it's just here to prove that you can connect between MySQL and MariaDB CS via Kafka. And that MariaDB ColumnStore works with Metabase for SQL queries.

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

# Observations

MariaDB ColumnStore is not optimized for writes, so it lags behind when there is a ton of data to insert. It is preferable to do INSERTs in batches. This becomes obvious over time if you look in Kafka UI for the consumer groups the lag for the analytics database.