package commands

import (
	"bytes"
	"io"
	"testing"
)

func TestBuildCommand(t *testing.T) {
	b := bytes.NewBufferString("")
	rootCmd.SetOut(b)
	rootCmd.SetErr(b)
	rootCmd.SetArgs([]string{"build"})
	rootCmd.Execute()
	out, err := io.ReadAll(b)
	if err != nil {
		t.Fatal(err)
	}
	if string(out) != "build called" {
		t.Fatalf("expected \"%s\" got \"%s\"", "build called", string(out))
	}
}
