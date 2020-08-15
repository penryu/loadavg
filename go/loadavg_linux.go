// +build linux

package main

import (
	"fmt"
	"io/ioutil"
	"path/filepath"
	"strconv"
	"strings"
)

func getTemp() (string, error) {
	var err error

	tempFiles, err := filepath.Glob("/sys/class/thermal/thermal_zone*/temp")
	if tempFiles == nil {
		err = fmt.Errorf("no thermal data found")
	}
	if err != nil {
		return "", err
	}

	var sum int64
	for _, tempFile := range tempFiles {
		data, err := ioutil.ReadFile(tempFile)
		if err != nil {
			return "", err
		}

		mdegsString := strings.TrimSpace(string(data))
		mdegs, err := strconv.ParseInt(mdegsString, 10, 64)
		if err != nil {
			return "", err
		}

		sum += mdegs
	}

	avgTemp := float64(sum) / float64(len(tempFiles))

	return fmt.Sprintf("%.1f°C", avgTemp/1000), nil
}
