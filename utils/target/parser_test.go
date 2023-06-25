package target

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestInvalidTargets(t *testing.T) {
	assert := assert.New(t)
	targets := []string{
		"hello world",
		"//",
		"//foo/bar...",
		"//foo/bar:baz:...",
		"foo/.../bar",
		"//foo ",
		"/hello",
		"hello/",
		"//foo/bar:baz/qux",
		"...:foo",
	}
	for _, target := range targets {
		_, err := parseTarget(target)

		assert.NotNil(err, fmt.Sprint("expected ", target, " to be invalid"))
	}
}

func TestRecursiveTargets(t *testing.T) {
	assert := assert.New(t)
	targets := []string{
		"...",
		":...",
		"//...",
		"//:...",
		"//foo/bar:...",
		"foo/bar:...",
	}
	for _, target := range targets {
		parsed, err := parseTarget(target)

		assert.Nil(err, fmt.Sprint("expected ", target, " to be valid, got error ", err))
		assert.Equal(parsed.name.kind, Recursive, fmt.Sprint("expected ", target, " to be recursive"))
	}
}

func TestSpecificTargets(t *testing.T) {
	assert := assert.New(t)
	tests := [][2]string{
		{"//foo", "foo"},
		{"//foo:bar", "bar"},
		{"foo", "foo"},
		{"foo:bar", "bar"},
		{"//foo/bar", "bar"},
		{"//foo/bar/baz", "baz"},
		{":hello", "hello"},
	}
	for _, test := range tests {
		target := test[0]
		expected := test[1]
		parsed, err := parseTarget(target)

		assert.Nil(err, fmt.Sprint("expected ", target, " to be valid, got error ", err))
		assert.Equal(parsed.name.kind, Specific, fmt.Sprint("expected ", target, " to be specific"))
		assert.Equal(parsed.name.value, expected, fmt.Sprint("expected ", target, " to have value ", expected, " got ", parsed.name.value))
	}
}

func TestAbsoluteTargets(t *testing.T) {
	assert := assert.New(t)
	targets := []string{
		"//...",
		"//:...",
		"//:hello",
		"//foo/bar:...",
		"//foo/bar:bar",
	}
	for _, target := range targets {
		parsed, err := parseTarget(target)

		assert.Nil(err, fmt.Sprint("expected ", target, " to be valid, got error ", err))
		assert.False(parsed.isRelative, fmt.Sprint("expected ", target, " to be absolute"))
	}
}

func TestRelativeTargets(t *testing.T) {
	assert := assert.New(t)
	targets := []string{
		"...",
		":...",
		":hello",
		"foo/bar:...",
		"foo/bar:bar",
	}
	for _, target := range targets {
		parsed, err := parseTarget(target)

		assert.Nil(err, fmt.Sprint("expected ", target, " to be valid, got error ", err))
		assert.True(parsed.isRelative, fmt.Sprint("expected ", target, " to be relative"))
	}
}

func TestTargetDirectories(t *testing.T) {
	assert := assert.New(t)
	tests := [][2][]string{
		{{"//foo"}, {"foo"}},
		{{"//foo:bar"}, {"foo"}},
		{{"foo"}, {"foo"}},
		{{"foo:bar"}, {"foo"}},
		{{"//foo/bar"}, {"foo", "bar"}},
		{{"//foo/bar/baz"}, {"foo", "bar", "baz"}},
		{{":hello"}, {}},
	}
	for _, test := range tests {
		target := test[0][0]
		expectedDirectories := test[1]
		parsed, err := parseTarget(target)

		assert.Nil(err, fmt.Sprint("expected ", target, " to be valid, got error ", err))
		assert.EqualValues(parsed.directories, expectedDirectories)

	}
}
