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
		_, err := ParseTarget(target)

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
		parsed, err := ParseTarget(target)

		assert.Nil(err, fmt.Sprint("expected ", target, " to be valid, got error ", err))
		assert.Equal(parsed.Name.Kind, Recursive, fmt.Sprint("expected ", target, " to be recursive"))
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
		parsed, err := ParseTarget(target)

		assert.Nil(err, fmt.Sprint("expected ", target, " to be valid, got error ", err))
		assert.Equal(parsed.Name.Kind, Specific, fmt.Sprint("expected ", target, " to be specific"))
		assert.Equal(parsed.Name.Value, expected, fmt.Sprint("expected ", target, " to have value ", expected, " got ", parsed.Name.Value))
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
		parsed, err := ParseTarget(target)

		assert.Nil(err, fmt.Sprint("expected ", target, " to be valid, got error ", err))
		assert.False(parsed.IsRelative, fmt.Sprint("expected ", target, " to be absolute"))
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
		parsed, err := ParseTarget(target)

		assert.Nil(err, fmt.Sprint("expected ", target, " to be valid, got error ", err))
		assert.True(parsed.IsRelative, fmt.Sprint("expected ", target, " to be relative"))
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
		parsed, err := ParseTarget(target)

		assert.Nil(err, fmt.Sprint("expected ", target, " to be valid, got error ", err))
		assert.EqualValues(parsed.Directories, expectedDirectories)

	}
}

func TestBuildFileLocation(t *testing.T) {
	assert := assert.New(t)
	tests := [][2]string{
		{"//foo", "foo/BUILD"},
		{"//foo:bar", "foo/BUILD"},
		{"foo", "foo/BUILD"},
		{"foo:bar", "foo/BUILD"},
		{"//foo/bar", "foo/bar/BUILD"},
	}
	for _, test := range tests {
		target := test[0]
		expected := test[1]
		parsed, err := ParseTarget(target)

		assert.Nil(err, fmt.Sprint("expected ", target, " to be valid, got error ", err))
		assert.Equal(parsed.BuildFileLocation(), expected, fmt.Sprint("expected ", target, " to have build file location ", expected, " got ", parsed.BuildFileLocation()))
	}
}

func TestTargetString(t *testing.T) {
	assert := assert.New(t)
	tests := [][2]string{
		{"//foo", "//foo:foo"},
		{"//foo:bar", "//foo:bar"},
		{"foo", "foo:foo"},
		{"foo:bar", "foo:bar"},
		{"//foo/bar", "//foo/bar:bar"},
	}
	for _, test := range tests {
		target := test[0]
		expected := test[1]
		parsed, err := ParseTarget(target)

		assert.Nil(err, fmt.Sprint("expected ", target, " to be valid, got error ", err))
		assert.Equal(parsed.ToString(), expected, fmt.Sprint("expected ", target, " to have string ", expected, " got ", parsed.ToString()))
	}
}
