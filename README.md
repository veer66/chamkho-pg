# chamkho-pg

chamkho-pg is a [pg-search-thai](https://github.com/zdk/pg-search-thai) port to Rust.


## Latest result

````
d4=# create extension chamkho_parser;
CREATE EXTENSION
d4=# CREATE TEXT SEARCH CONFIGURATION chamkho (PARSER = chamkho_parser);
CREATE TEXT SEARCH CONFIGURATION
d4=# ALTER TEXT SEARCH CONFIGURATION chamkho ADD MAPPING FOR word WITH simple;
ALTER TEXT SEARCH CONFIGURATION
d4=# select to_tsvector('thai', 'ฉันกินข้าว');
      to_tsvector      
-----------------------
 'กิน':2 'ข้าว':3 'ฉัน':1
(1 row)
````

