set -e

cargo install --path .

sh tests/default.sh
sh tests/in-sub-dir.sh
sh tests/not-git-dir.sh