# chamkho-pg

_chamkho-pg_ is a Rust port of [pg-search-thai](https://github.com/zdk/pg-search-thai). It's a PostgreSQL extension whose objective is to enable full-text searching on Southeast Asian and other languages. Currently, chamkho-pg supports Chinese, Japanese, Khmer, Myanmar, Lao, Thai, and other space-delimited languages.

## How to install

1. git clone https://github.com/veer66/chamkho-pg.git
2. cd chamkho-pg
3. cargo build --release
4. ./install.sh

## Latest result

```
d4=# create extension chamkho_parser;
CREATE EXTENSION
d4=# CREATE TEXT SEARCH CONFIGURATION chamkho (PARSER = chamkho_parser);
CREATE TEXT SEARCH CONFIGURATION
d4=# ALTER TEXT SEARCH CONFIGURATION chamkho ADD MAPPING FOR word WITH simple;
ALTER TEXT SEARCH CONFIGURATION
d4=# select to_tsvector('chamkho',
  'វេវចនានុក្រមពហុភាសាដោយឥតគិតថ្លៃฉันกินข้าวຈະຊອກຫາອີ່ຫຍັງ本日のお仕事終了しましたpop musicရှေးန်မာမင်း\0 အဆက်ဆက်ကတည်း');
to_tsvector
'0':30 '\\':29 'music':23 'pop':22 'กินข้าว':9 'ฉัน':8 'ຈະ':10 'ຊອກ':11 'ຫຍັງ':15 'ຫາ':12 'ອີ':13 '່':14
'က':32 'တည်း':33 'န':25 'မင်း':28 'မာ':27 'ရှေး':24 'အဆက်ဆက်':31 '်':26 'គិតថ្លៃ':7 'ដោយ':5 'ពហុ':3
'ភាសា':4 'វចនានុក្រម':2 'វេ':1 'ឥត':6 'お仕事':18 'した':21 'しま':20 'の':17 '本日':16 '終了':19
(1 row)
```

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
