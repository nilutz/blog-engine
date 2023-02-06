SHELL := /bin/bash

make_zip:
	cargo run && rm -rf publish.zip && zip -r publish.zip public/ -x '.??*' -x '*.DS_Store'

publish_with_wrangler:
	cargo run && yarn wrangler
