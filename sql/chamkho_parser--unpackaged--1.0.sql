-- complain if script is sourced in psql, rather than via CREATE EXTENSION
\echo Use "CREATE EXTENSION chamkho_parser" to load this file. \quit

ALTER EXTENSION chamkho_parser ADD function chamkho_parser_start(internal,integer);
ALTER EXTENSION chamkho_parser ADD function chamkho_parser_get_token(internal,internal,internal);
ALTER EXTENSION chamkho_parser ADD function chamkho_parser_end(internal);
ALTER EXTENSION chamkho_parser ADD text search parser chamkho_parser;
