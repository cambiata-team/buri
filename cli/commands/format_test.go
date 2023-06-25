package commands

import (
	"bytes"
	"io"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestFormatCommand(t *testing.T) {
	assert := assert.New(t)
	b := bytes.NewBufferString("")
	rootCmd.SetOut(b)
	rootCmd.SetErr(b)
	rootCmd.SetArgs([]string{"format"})

	assert.Nil(rootCmd.Execute())

	out, err := io.ReadAll(b)
	assert.NotNil(err)
	assert.Equal("format called", string(out))
}
