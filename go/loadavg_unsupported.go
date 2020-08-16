// +build !darwin
// +build !linux

package main

import (
	"errors"
	"runtime"
)

func getTemp() (string, error) {
	err := errors.New("temperature unsupported on platform: " + runtime.GOOS)

	return "", err
}
