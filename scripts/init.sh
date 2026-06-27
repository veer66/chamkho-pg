#!/bin/sh

project_dir=$(git rev-parse --show-toplevel)
install -m 0755 $project_dir/scripts/hooks/pre-commit $project_dir/.git/hooks/
