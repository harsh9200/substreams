.PHONY: build_all
build_all:
	$(MAKE) -C ../erc20-price build
	$(MAKE) -C ../erc20-price pack
	cargo build --target wasm32-unknown-unknown --release

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: run
run:
	substreams run -e polygon.streamingfast.io:443 substreams.yaml map_pool_fees --debug-modules-initial-snapshot store_pools -s 13991900 -t +10

.PHONY: build_csv
build_csv:
	substreams-sink-csv -e polygon.substreams.pinax.network:9000 --manifest ./curve-finance-substream-v0.1.0.spkg --module-name map_pool_fees --start-block 13991900

.PHONY: pack
pack:
	substreams pack ./substreams.yaml
