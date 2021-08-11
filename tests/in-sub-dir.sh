. "$(dirname "$0")/functions.sh"
setup

# Test custom dir support
mkdir sub
captain-hook install sub/captain-hook
captain-hook add sub/captain-hook/pre-commit "echo \"pre-commit\" && exit 1"

# Test core.hooksPath
expect_hooksPath_to_be "sub/captain-hook"

# Test pre-commit
git add Cargo.toml
expect 1 "git commit -m foo"