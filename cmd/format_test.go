package cmd

import (
	"bytes"
	"io"
	"testing"
)

func TestFormatCommand(t *testing.T) {
	b := bytes.NewBufferString("")
	rootCmd.SetOut(b)
	rootCmd.SetErr(b)
	rootCmd.SetArgs([]string{"format"})
	rootCmd.Execute()
	out, err := io.ReadAll(b)
	if err != nil {
		t.Fatal(err)
	}
	if string(out) != "format called" {
		t.Fatalf("expected \"%s\" got \"%s\"", "format called", string(out))
	}
}
