#!/bin/sh
python3 ./simple_sync.py
git add -u
git commit -m "automated checkpoint after simple_sync"
./clean.sh

python3 ./simple_concurrent.py
git add -u
git commit -m "automated checkpoint after simple_concurrent"
./clean.sh

python3 ./graph_sync.py
python3 ./graph_concurrent.py
