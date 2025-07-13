package main

import (
	"flag"
	"io"
	"log"
	"os"
	"os/exec"
	"time"

	"golang.design/x/clipboard"
)

func main() {
	// Initialize the clipboard.
	err := clipboard.Init()
	if err != nil {
		log.Fatal(err)
	}

	// Read from stdin.
	input, err := io.ReadAll(os.Stdin)
	if err != nil {
		log.Fatal(err)
	}

	// Write to clipboard.
	clipboard.Write(clipboard.FmtText, input)

}
