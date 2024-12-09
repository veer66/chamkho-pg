#!/bin/sh

# Copy the library with the correct extension
if [ "$(uname)" = "Darwin" ]; then
    # Depending on the brew formula version, the expected extension can be either .dylib or .so
    cp "target/release/libchamkho_parser.dylib" "$(pg_config --libdir)/postgresql/chamkho_parser.dylib"
    ln -sf "$(pg_config --libdir)/postgresql/chamkho_parser.dylib" "$(pg_config --libdir)/postgresql/chamkho_parser.so"
else
    cp "target/release/libchamkho_parser.so" "$(pg_config --libdir)/postgresql/chamkho_parser.so"
fi

# Copy extension files
cp control/*.control sql/*.sql "$(pg_config --sharedir)/extension"
