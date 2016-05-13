## Macros

# Program Name
PROJECT = loadavg

BINDIR = $(HOME)/bin

CFLAGS += -g -m64 -std=c99 $(SAFE_C)
SAFE_C = -Wall -Wextra -Wconversion -pedantic \
	-fno-builtin -Wno-unused-variable

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
	$(CC) $(CFLAGS) -c $<
