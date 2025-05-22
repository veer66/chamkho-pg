# chamkho-pg

_chamkho-pg_ is a Rust port of [pg-search-thai](https://github.com/zdk/pg-search-thai). It's a PostgreSQL extension whose objective is to enable full-text searching on Southeast Asian and other languages. Currently, chamkho-pg supports Chinese, Japanese, Khmer, Myanmar, Lao, Shan, Thai, and other space-delimited languages.

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
  'វេវចនានុក្រមពហុភាសាដោយឥតគិតថ្លៃฉันกินข้าวၵႂၢမ်းတႆးလိၵ်ႈတႆးຈະຊອກຫາອີ່ຫຍັງ本日のお仕事終了しましたpop musicရှေးန်မာမင်း အဆက်ဆက်ကတည်း');
to_tsvector
'music':27 'pop':26 'กินข้าว':9 'ฉัน':8 'ຈະ':14 'ຊອກ':15 'ຫຍັງ':19 'ຫາ':16 'ອີ':17 '່':18 'က':34 'တည်း:35 'တႆး:11,13 'န':29
 'မင်း:32 'မာ':31 'ရှေး28 'လိၵ်ႈ:12 'အဆက်ဆက်':33 '်':30 'ၵႂၢ':10 'គិតថ្លៃ':7 'ដោយ':5 'ពហុ':3 'ភាសា':4 'វចនានុក្រម':2 'វេ':1 '
ឥត':6 'お仕事':22 'した':25 'しま':24 'の':21 '本日':20 '終了':23
(1 row)
```

## Talk

PostgreSQL Extension for Full-Text Search in Any Language  https://youtube.com/watch?v=C6o71HQXTaQ

## Status

chamkho-pg currently supports PostgreSQL 15 and 16 on both GNU/Linux and macOS.

Current Docker images are built with PostgreSQL 16.6.

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
