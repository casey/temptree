bt := '0'

export RUST_BACKTRACE := bt

version := `sed -En 's/version[[:space:]]*=[[:space:]]*"([^"]+)"/v\1/p' Cargo.toml | head -1`

watch:
	cargo watch --clear --exec ltest

test:
	cargo test

forbid:
	./bin/forbid

publish-check: clippy test forbid
	git branch | grep '* master'
	git diff --no-ext-diff --quiet --exit-code
	grep {{version}} CHANGELOG.md
	cargo test

publish: publish-check
	cargo publish
	git tag -a {{version}} -m 'Release {{version}}'
	git push github {{version}}

# clean up feature branch BRANCH
done BRANCH:
	git checkout master
	git diff --no-ext-diff --quiet --exit-code
	git pull --rebase github master
	git diff --no-ext-diff --quiet --exit-code {{BRANCH}}
	git branch -D {{BRANCH}}

# everyone's favorite animate paper clip
clippy:
	cargo clippy
