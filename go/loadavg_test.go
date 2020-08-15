package main

import (
	"regexp"
	"testing"
)

var (
	validTemp    = regexp.MustCompile(`^(?:\d+)\.(?:\d+)°C$`)
	validLoadAvg = regexp.MustCompile(`^(\d+\.\d+)(?: (\d+\.\d+)){2}$`)
)

func TestGetLoadAvg(t *testing.T) {
	loadavgOutput := getLoadAvg()

	if !validLoadAvg.MatchString(loadavgOutput) {
		t.Errorf("Invalid load average output: %s", loadavgOutput)
	}
}

func TestGetTemp(t *testing.T) {
	tempOutput, err := getTemp()

	if err != nil {
		t.Skip(err)
	}

	if !validTemp.MatchString(tempOutput) {
		t.Errorf("malformed temperature: %s", tempOutput)
	}

}
