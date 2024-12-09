#!/bin/sh

# Determine the platform-specific library extension
if [ "$(uname)" = "Darwin" ]; then
    LIB_EXT="dylib"
else
    LIB_EXT="so"
fi

# Copy the library with the correct extension
cp "target/release/libchamkho_parser.$LIB_EXT" "$(pg_config --libdir)/postgresql/chamkho_parser.so"

# Copy extension files
cp control/*.control sql/*.sql "$(pg_config --sharedir)/extension"
