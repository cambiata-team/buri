package commands

import (
	"bytes"
	"io"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestTestCommand(t *testing.T) {
	assert := assert.New(t)
	b := bytes.NewBufferString("")
	rootCmd.SetOut(b)
	rootCmd.SetErr(b)
	rootCmd.SetArgs([]string{"test"})

	assert.Nil(rootCmd.Execute())

	out, err := io.ReadAll(b)
	assert.Nil(err)
	assert.Equal("test called", string(out))
}
