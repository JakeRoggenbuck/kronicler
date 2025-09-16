p ./simple_sync.py
git add -u
git commit -m "automated checkpoint after simple_sync"
./clean.sh

p ./simple_concurrent.py
git add -u
git commit -m "automated checkpoint after simple_concurrent"
./clean.sh

p ./graph_sync.py
p ./graph_concurrent.py
