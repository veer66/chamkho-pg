DROP TEXT SEARCH PARSER chamkho_parser;
DROP FUNCTION chamkho_parser_start(internal, int4);
DROP FUNCTION chamkho_parser_get_token(internal, internal, internal);
DROP FUNCTION chamkho_parser_end(internal);
DROP FUNCTION chamkho_parser_lextype(internal);

CREATE FUNCTION chamkho_parser_start(internal, int4)
RETURNS internal
AS '/home/vee/Develop/free/chamkho-pg/target/release/libchamkho_pg.so'
LANGUAGE C STRICT;

CREATE FUNCTION chamkho_parser_get_token(internal, internal, internal)
RETURNS internal
AS '/home/vee/Develop/free/chamkho-pg/target/release/libchamkho_pg.so'
LANGUAGE C STRICT;

CREATE FUNCTION chamkho_parser_end(internal)
RETURNS void
AS '/home/vee/Develop/free/chamkho-pg/target/release/libchamkho_pg.so'
LANGUAGE C STRICT;

CREATE FUNCTION chamkho_parser_lextype(internal)
RETURNS internal
AS '/home/vee/Develop/free/chamkho-pg/target/release/libchamkho_pg.so'
LANGUAGE C STRICT;



CREATE TEXT SEARCH PARSER chamkho_parser (
    START    = chamkho_parser_start,
    GETTOKEN = chamkho_parser_get_token,
    END      = chamkho_parser_end,
    HEADLINE = pg_catalog.prsd_headline,
    LEXTYPES = chamkho_parser_lextype
);
