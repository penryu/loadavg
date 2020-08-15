BIN = loadavg

DEPS = *.go go.mod

all: $(BIN)

test: $(DEPS)
	go test

clean:
	go clean

$(BIN): $(DEPS)
	go build
