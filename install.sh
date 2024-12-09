#!/bin/sh

cp target/release/libchamkho_parser.dylib `pg_config --libdir`/postgresql/chamkho_parser.so
cp control/*.control sql/*.sql `pg_config --sharedir`/extension
