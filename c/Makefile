# Macros
BIN = loadavg
PRODUCTS = $(BIN) $(BIN).dSYM
CFLAGS += -g -O0 -Werror -Wall -Wextra -Wconversion -pedantic -fno-builtin

# Targets
all: $(BIN)

install: $(BIN)
	install -s $(BIN) /usr/local/bin/$(BIN)

test: $(BIN)
	./$(BIN)

clean:
	rm -rf $(PRODUCTS)

