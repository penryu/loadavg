package main

// #include <stdlib.h>
import "C"

import (
	"fmt"
	"log"
)

const maxSamples = 3

func main() {
	var ls [maxSamples]C.double

	n := C.getloadavg(&ls[0], maxSamples)
	if n < maxSamples {
		// we didn't get enough samples
		log.Fatalf("Insufficient samples: %d\n", n)
	}

	fmt.Printf("%03.2f %03.2f %03.2f\n", ls[0], ls[1], ls[2])
}
