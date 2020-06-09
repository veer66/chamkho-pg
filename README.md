# chamkho-pg

_chamkho-pg_ (Thai: _ชำฆ้อพีจี_) is a [pg-search-thai](https://github.com/zdk/pg-search-thai) port to Rust. _chamkho-pg_ is a PostgreSQL extension, which its objective is enabling PostgreSQL full-text searching on SE Asian languages. Currently, _chamkho-pg_ supports Lao and Thai.


## Latest result

````
d4=# create extension chamkho_parser;
CREATE EXTENSION
d4=# CREATE TEXT SEARCH CONFIGURATION chamkho (PARSER = chamkho_parser);
CREATE TEXT SEARCH CONFIGURATION
d4=# ALTER TEXT SEARCH CONFIGURATION chamkho ADD MAPPING FOR word WITH simple;
ALTER TEXT SEARCH CONFIGURATION
d4=# select to_tsvector('chamkho', 'ฉันกินข้าวຈະຊອກຫາອີ່ຫຍັງ');
                          to_tsvector                           
----------------------------------------------------------------
 'กิน':2 'ข้าว':3 'ฉัน':1 'ຈະ':4 'ຊອກ':5 'ຫຍັງ':9 'ຫາ':6 'ອີ':7 '່':8
(1 row)
````

