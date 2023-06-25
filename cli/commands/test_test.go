package commands

import (
	"bytes"
	"io"
	"testing"
)

func TestTestCommand(t *testing.T) {
	b := bytes.NewBufferString("")
	rootCmd.SetOut(b)
	rootCmd.SetErr(b)
	rootCmd.SetArgs([]string{"test"})
	rootCmd.Execute()
	out, err := io.ReadAll(b)
	if err != nil {
		t.Fatal(err)
	}
	if string(out) != "test called" {
		t.Fatalf("expected \"%s\" got \"%s\"", "test called", string(out))
	}
}
