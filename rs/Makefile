BIN = loadavg

DEPS = src/main.rs

PRODUCTS = ./target/ loadavg

all: $(BIN)

run: $(BIN)
	./$(BIN)

clean:
	cargo clean
	rm -rf $(PRODUCTS)

$(BIN): $(DEPS)
	cargo build --release
	cp target/release/loadavg .
