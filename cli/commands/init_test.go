package commands

import (
	"bytes"
	"io"
	"testing"

	"github.com/spf13/afero"
	"github.com/stretchr/testify/assert"
)

func TestInitCreatesWorkspaceFile(t *testing.T) {
	assert := assert.New(t)
	fs := afero.NewMemMapFs()

	initCommand := NewInitCommand(fs)
	stdOut := bytes.NewBufferString("")
	initCommand.SetOut(stdOut)

	err := initCommand.Execute()
	assert.Nil(err)

	afs := &afero.Afero{Fs: fs}
	exists, err := afs.Exists("WORKSPACE")
	assert.Nil(err)
	assert.True(exists)
}

func TestInitLogsDoneWhenFinished(t *testing.T) {
	assert := assert.New(t)
	fs := afero.NewMemMapFs()

	stdOut := bytes.NewBufferString("")
	initCommand := NewInitCommand(fs)
	initCommand.SetOut(stdOut)

	err := initCommand.Execute()
	assert.Nil(err)

	out, err := io.ReadAll(stdOut)
	assert.Nil(err)
	assert.Equal("done", string(out))

}

func TestWorkspaceFileContainsName(t *testing.T) {
	assert := assert.New(t)
	fs := afero.NewMemMapFs()

	initCommand := NewInitCommand(fs)
	stdOut := bytes.NewBufferString("")
	initCommand.SetOut(stdOut)
	initCommand.SetArgs([]string{"--name", "my-workspace"})

	err := initCommand.Execute()
	assert.Nil(err)

	afs := &afero.Afero{Fs: fs}
	workspaceContents, err := afs.ReadFile("WORKSPACE")
	assert.Nil(err)
	// Two separate assertions because the textproto format is not guaranteed to be stable
	assert.Contains(string(workspaceContents), "name: ")
	assert.Contains(string(workspaceContents), "\"my-workspace\"")
}

func TestInitDoesNotOverwriteExistingWorkspaceFile(t *testing.T) {
	assert := assert.New(t)
	fs := afero.NewMemMapFs()
	afs := &afero.Afero{Fs: fs}
	err := afs.WriteFile("WORKSPACE", []byte("workspace contents"), 0644)
	assert.Nil(err)

	initCommand := NewInitCommand(fs)
	stdOut := bytes.NewBufferString("")
	initCommand.SetOut(stdOut)

	err = initCommand.Execute()
	assert.Nil(err)

	out, err := io.ReadAll(stdOut)
	assert.Nil(err)
	assert.Contains(string(out), "error: Could not create a new workspace because workspace file already exists")
}
