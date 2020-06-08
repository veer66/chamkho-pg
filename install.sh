#!/bin/sh

cp target/release/libchamkho_parser.so `pg_config --libdir`/postgresql/chamkho_parser.so
cp control/*.control sql/*.sql `pg_config --sharedir`/extension
