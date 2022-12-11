Simple 
# Instructions
```> cargo pgx run --release
$> CREATE EXTENSION fastring;
$> \timing
$> SELECT COUNT(hello_unguarded_void()) FROM generate_series(1,10000000);
$> SELECT COUNT(hello_void()) FROM generate_series(1,10000000);
$> SELECT COUNT(hello_32()) FROM generate_series(1,10000000);
$> SELECT COUNT(hello_str()) FROM generate_series(1,10000000);
$> SELECT COUNT(hello_fastr()) FROM generate_series(1,10000000);
