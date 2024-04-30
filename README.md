# chamkho-pg

_chamkho-pg_ (Thai: _ชำฆ้อพีจี_) is a [pg-search-thai](https://github.com/zdk/pg-search-thai) port to Rust. _chamkho-pg_ is a PostgreSQL extension, which its objective is enabling PostgreSQL full-text searching on SE Asian languages. Currently, _chamkho-pg_ supports Lao and Thai.

## How to install

1. git clone https://github.com/veer66/chamkho-pg.git
2. cd chamkho-pg
3. cargo build --release
4. ./install.sh

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

## Status

chamkho-pg currently support PostgreSQL 15 on GNU/Linux.

## Example

### Initiailize

```
create extension chamkho_parser;
CREATE TEXT SEARCH CONFIGURATION chamkho (PARSER = chamkho_parser);
ALTER TEXT SEARCH CONFIGURATION chamkho ADD MAPPING FOR word WITH simple;
```

### Prepare table

```
create table tab1(id serial, body text);
insert into tab1(body) values ('ไก่กับเป็ด'), ('ช้างม้า'), ('วัวหมี');
```

### Query

```
select * from tab1 where to_tsvector('chamkho', body) @@ to_tsquery('เป็ด & ไก่');
```

### Index

```
CREATE INDEX tab1_idx ON tab1 USING GIN (to_tsvector('chamkho', body));
```

## Podman

### Build

```
$ git clone https://github.com/veer66/chamkho-pg.git
$ cd chamkho-pg
$ podman build -t chamkho-pg .
```

### Run

```
$ podman run --name chamkho-pg-1 -e POSTGRES_PASSWORD=yourpass -d chamkho-pg
```

### Use

```
$ podman exec -it chamkho-pg-1 psql -U postgres
```
