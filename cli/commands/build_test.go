package commands

import (
	"bytes"
	"io"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestBuildCommand(t *testing.T) {
	assert := assert.New(t)
	b := bytes.NewBufferString("")
	rootCmd.SetOut(b)
	rootCmd.SetErr(b)
	rootCmd.SetArgs([]string{"build"})

	assert.Nil(rootCmd.Execute())

	out, err := io.ReadAll(b)
	assert.NotNil(err)
	assert.Equal("build called", string(out))
}
