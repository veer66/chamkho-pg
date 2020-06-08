CREATE FUNCTION chamkho_parser_start(internal, int4)
RETURNS internal
AS 'MODULE_PATHNAME'
LANGUAGE C STRICT;

CREATE FUNCTION chamkho_parser_get_token(internal, internal, internal)
RETURNS internal
AS 'MODULE_PATHNAME'
LANGUAGE C STRICT;

CREATE FUNCTION chamkho_parser_end(internal)
RETURNS void
AS 'MODULE_PATHNAME'
LANGUAGE C STRICT;

CREATE TEXT SEARCH PARSER chamkho_parser (
    START    = chamkho_parser_start,
    GETTOKEN = chamkho_parser_get_token,
    END      = chamkho_parser_end,
    HEADLINE = pg_catalog.prsd_headline,
    LEXTYPES = prsd_lextype
);
