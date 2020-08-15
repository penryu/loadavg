// +build darwin

package main

import (
	"bytes"
	"os/exec"
	"strings"
)

func getTemp() (string, error) {
	cmdPath, err := exec.LookPath("osx-cpu-temp")
	if err != nil {
		// no supported source of cpu temp found
		return "", err
	}

	var cmdOut bytes.Buffer
	cmd := exec.Command(cmdPath)
	cmd.Stdout = &cmdOut
	err = cmd.Run()

	if err != nil {
		// The utility returned non-zero
		return "", err
	}

	tempC := strings.ReplaceAll(cmdOut.String(), "\n", " ")
	return strings.TrimSpace(tempC), nil
}
