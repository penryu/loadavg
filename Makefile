# Macros
PROJECT = loadavg
SRC = $(BIN).c
BIN = $(PROJECT)
PRODUCTS = $(BIN) $(BIN).dSYM $(BIN).out

CFLAGS += -g -O0 -Wall -Wextra -Wconversion -pedantic -fno-builtin

# Targets
all: $(BIN)

run: $(BIN)
	./$(BIN) | tee $(BIN).out

clean:
	rm -rf $(PRODUCTS)

$(BIN): $(SRC)
	$(CC) $(CFLAGS) $(SRC) -o $(BIN)
