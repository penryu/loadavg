## Macros

# Program Name
PROJECT = loadavg

CC = gcc
CFLAGS = -g -m64 -Wall -Wextra -Wconversion -pedantic

BIN = $(PROJECT)
OBJS =


# Target

all: $(BIN)

run: $(BIN)
	./$(BIN)

clean:
	rm -rf $(OBJS) $(BIN) $(BIN).dSYM
	rm -rf $(EXPORT_DIR)

$(BIN): $(OBJS) $(BIN).c
	$(CC) $(CFLAGS) -o $(BIN) $(BIN).c $(OBJS)

%.o: %.c %.h
	$(C) $(CFLAGS) -c $<
