package main

// #include <stdlib.h>
import "C"

import (
	"fmt"
	"log"
	"strings"
)

const (
	maxSamples = 3
	separator  = " / "
)

func getLoadAvg() string {

	var ls [maxSamples]C.double

	n := C.getloadavg(&ls[0], maxSamples)
	if n < maxSamples {
		// we didn't get enough samples
		log.Fatalf("Insufficient samples: %d\n", n)
	}

	return fmt.Sprintf("%03.2f %03.2f %03.2f", ls[0], ls[1], ls[2])
}

func main() {
	var output strings.Builder

	output.WriteString(getLoadAvg())

	if tempC, _ := getTemp(); tempC != "" {
		output.WriteString(separator + tempC)
	}

	fmt.Println(output.String())
}
