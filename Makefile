build:
	cargo build
	cd examples/helloworld && make
	cd examples/malloc && make
	cd examples/canvas && make
serve:
	http-server -p 8080
