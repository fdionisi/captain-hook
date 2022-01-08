. "$(dirname "$0")/functions.sh"
setup

captain-hook install

# Test core.hooksPath
expect_hooksPath_to_be ".husky"

# Test pre-commit
git add Cargo.toml
captain-hook add .husky/pre-commit "echo \"pre-commit\" && exit 1"
expect 1 "git commit -m foo"

# Uninstall
captain-hook uninstall
expect 1 "git config core.hooksPath"