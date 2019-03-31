#!/bin/sh

# Generate fresh bindings TSV file to use.
bindgen ompt.h --default-enum-style=rust --no-prepend-enum-name > ompt.rs
grep -E -o '(ompt|omp)_[a-z_]*' ompt.rs | sort | uniq > basenames.tsv
paste basenames.tsv basenames.tsv > ompt.tsv
python3 rustify.py ompt.tsv > final.tsv
cat ompt.rs | python3 cleanup.py final.tsv > ompt-final.rs
echo "use std::os::raw::*;" | cat - ompt-final.rs > ompt.rs
