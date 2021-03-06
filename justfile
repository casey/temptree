bt := '0'

export RUST_BACKTRACE := bt

version := `sed -En 's/version[[:space:]]*=[[:space:]]*"([^"]+)"/v\1/p' Cargo.toml | head -1`

watch:
	cargo watch --clear --exec test

test:
	cargo test

publish-check: lint clippy test
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

@lint:
	echo Checking for FIXME/TODO...
	! grep --color -En 'FIXME|TODO' src/*.rs
	echo Checking for long lines...
	! grep --color -En '.{101}' src/*.rs
