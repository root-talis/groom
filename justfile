# Shows list of recipes
default:
    just --list

# Install version-controlled git hooks (pre-push semver tag checks)
install-hooks:
    git config core.hooksPath .githooks

# Run crate tests and build every example
test:
    #!/usr/bin/env sh
    set -eu

    cargo test -p groom
    cargo test -p groom_macros

    for example in examples/*/; do
        if [ -f "${example}justfile" ]; then
            just --justfile "${example}justfile" test
        fi
    done
