#!/bin/bash
set -e

# Install the extension via pgrx
cargo pgrx install --release "$@"

# Get PostgreSQL share directory
PG_SHARE_DIR=$(pg_config --sharedir)

# Install dictionary data
echo "Installing dictionary data to ${PG_SHARE_DIR}/tsearch_data/"
mkdir -p "${PG_SHARE_DIR}/tsearch_data"
cp tsearch_data/chamkho_dict.txt "${PG_SHARE_DIR}/tsearch_data/"

echo "Installation complete."
