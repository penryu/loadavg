# Macros
BIN = loadavg
DBG_BIN = $(BIN)-dbg

PRODUCTS = $(BIN) $(DBG_BIN) *.dSYM
CFLAGS += -O3 -Werror -Wall -Wextra -Wconversion -pedantic -fno-builtin

# Targets
all: $(BIN)
	strip $(BIN)

debug: $(BIN)-dbg

install: $(BIN)
	install -s $(BIN) /usr/local/bin/$(BIN)

test: $(BIN)
	./$(BIN)

clean:
	rm -rf $(PRODUCTS)

$(DBG_BIN): $(BIN).c
	$(CC) $(CFLAGS) -g -O0 -v $< -o $@
