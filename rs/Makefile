BIN = loadavg

DEPS = src/main.rs

PRODUCTS = ./target/

all: $(BIN)
	cp target/release/loadavg .

test: $(DEPS)
	cargo test

clean:
	cargo clean

$(BIN): $(DEPS)
	cargo build --release
