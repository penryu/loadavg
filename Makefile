## Macros

# Program Name
PROJECT = loadavg

BINDIR = $(HOME)/bin

# CC = gcc
CXXFLAGS += -g -O0 -m64 -std=c++11 $(SAFE_CXX)
SAFE_CXX = -Wall -Wextra -Wconversion -Weffc++ -pedantic \
	-fno-builtin \
	-Wno-unused-variable

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

$(BIN): $(OBJS) $(BIN).cc
	$(CXX) $(CXXFLAGS) -o $(BIN) $(BIN).cc $(OBJS)

%.o: %.c %.h
	$(C) $(CFLAGS) -c $<
