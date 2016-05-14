# Macros
PROJECT = loadavg
BIN = $(PROJECT)
SRC = $(BIN).c

CFLAGS += -Wall -Wextra -Wconversion -pedantic -fno-builtin

# Targets
all: $(BIN)

run: $(BIN)
	./$(BIN) | tee $(BIN).out

clean:
	rm -rf $(BIN) $(BIN).dSYM $(BIN).out

$(BIN): $(SRC)
	$(CC) $(CFLAGS) $(SRC) -o $(BIN)
