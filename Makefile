## Macros

# Program Name
PROJECT = loadavg

BINDIR = $(HOME)/bin

CC = gcc
CFLAGS = -g -m64 -Wall -Wextra -Wconversion -pedantic

BIN = $(PROJECT)
OBJS =


# Target

all: $(BIN)

run: $(BIN)
	./$(BIN)

install: $(BIN)
	mkdir -p $(BINDIR)
	install -m755 -s $(BIN) $(BINDIR)

clean:
	rm -rf $(OBJS) $(BIN) $(BIN).dSYM
	rm -rf $(EXPORT_DIR)

$(BIN): $(OBJS) $(BIN).c
	$(CC) $(CFLAGS) -o $(BIN) $(BIN).c $(OBJS)

%.o: %.c %.h
	$(C) $(CFLAGS) -c $<
