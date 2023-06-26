package helpers

import (
	"buri/utils/target"
	"testing"

	"github.com/spf13/afero"
	"github.com/stretchr/testify/assert"
)

func TestErrorsIfBuildFileDoesNotExist(t *testing.T) {
	assert := assert.New(t)

	headTarget, _ := target.ParseTarget("//foo:bar")
	afs := &afero.Afero{Fs: afero.NewMemMapFs()}

	_, err := ResolveDepGraph(&headTarget, afs)

	assert.NotNil(err)
}

func TestErrorsIfBuildFileDoesNotContainTarget(t *testing.T) {
	assert := assert.New(t)

	headTarget, _ := target.ParseTarget("//foo:bar")
	afs := &afero.Afero{Fs: afero.NewMemMapFs()}
	err := afs.WriteFile("foo/BUILD", []byte(""), 0644)
	assert.Nil(err)

	depGraph, err := ResolveDepGraph(&headTarget, afs)

	assert.NotNil(err)

	assert.Equal(0, len(depGraph.Deps))
}

func TestDepGraphNodeHasNoChildrenIfTargetHasNoDependencies(t *testing.T) {
	assert := assert.New(t)

	headTarget, _ := target.ParseTarget("//foo:bar")
	afs := &afero.Afero{Fs: afero.NewMemMapFs()}
	err := afs.WriteFile("foo/BUILD", []byte(
		`
	library {
		name: "bar"
	}
	`), 0644)
	assert.Nil(err)

	depGraph, err := ResolveDepGraph(&headTarget, afs)

	assert.Nil(err)
	assert.Equal(0, len(depGraph.Deps))
}

func TestErrorsIfDependencyBuildFileDoesNotExist(t *testing.T) {
	assert := assert.New(t)

	headTarget, _ := target.ParseTarget("//foo:bar")
	afs := &afero.Afero{Fs: afero.NewMemMapFs()}
	err := afs.WriteFile("foo/BUILD", []byte(
		`
	library {
		name: "bar"
		deps: ["//fizz/buzz:qux"]
	}
	`), 0644)
	assert.Nil(err)

	_, err = ResolveDepGraph(&headTarget, afs)

	assert.NotNil(err)
}

func TestErrorsDependencyBuildFileDoesNotIncludeTarget(t *testing.T) {
	assert := assert.New(t)

	headTarget, _ := target.ParseTarget("//foo:bar")
	afs := &afero.Afero{Fs: afero.NewMemMapFs()}
	err := afs.WriteFile("foo/BUILD", []byte(
		`
	library {
		name: "bar"
		dependencies: ["//fizz/buzz:qux"]
	}
	`), 0644)
	assert.Nil(err)
	err = afs.WriteFile("fizz/buzz/BUILD", []byte(
		`
	library {
		name: "foo"
	}
	`), 0644)
	assert.Nil(err)

	_, err = ResolveDepGraph(&headTarget, afs)

	assert.NotNil(err)
}

func TestDependencyAddedToDepGraph(t *testing.T) {
	assert := assert.New(t)

	headTarget, _ := target.ParseTarget("//foo:bar")
	afs := &afero.Afero{Fs: afero.NewMemMapFs()}
	err := afs.WriteFile("foo/BUILD", []byte(
		`
	library {
		name: "bar"
		dependencies: ["//fizz/buzz:qux"]
	}
	`), 0644)
	assert.Nil(err)
	err = afs.WriteFile("fizz/buzz/BUILD", []byte(
		`
	library {
		name: "qux"
	}
	`), 0644)
	assert.Nil(err)

	head, err := ResolveDepGraph(&headTarget, afs)

	assert.Nil(err)
	assert.Equal(1, len(head.Deps))
}

func TestTraversesMultipleDependencyBuildFiles(t *testing.T) {
	assert := assert.New(t)

	headTarget, _ := target.ParseTarget("//foo:bar")
	afs := &afero.Afero{Fs: afero.NewMemMapFs()}
	err := afs.WriteFile("foo/BUILD", []byte(
		`
	library {
		name: "bar"
		dependencies: ["//fizz/buzz:qux", "//hello:world"]
	}
	`), 0644)
	assert.Nil(err)
	err = afs.WriteFile("fizz/buzz/BUILD", []byte(
		`
	library {
		name: "qux"
	}
	`), 0644)
	assert.Nil(err)
	err = afs.WriteFile("hello/BUILD", []byte(
		`
	library {
		name: "world"
	}
	`), 0644)
	assert.Nil(err)

	head, err := ResolveDepGraph(&headTarget, afs)

	assert.Nil(err)
	assert.Equal(2, len(head.Deps))
}

func TestTraversesMultipleTargetsInSameBuildFile(t *testing.T) {
	assert := assert.New(t)

	headTarget, _ := target.ParseTarget("//foo:bar")
	afs := &afero.Afero{Fs: afero.NewMemMapFs()}
	err := afs.WriteFile("foo/BUILD", []byte(
		`
	library {
		name: "bar"
		dependencies: ["//fizz/buzz:qux", "//fizz/buzz:hello"]
	}
	`), 0644)
	assert.Nil(err)
	err = afs.WriteFile("fizz/buzz/BUILD", []byte(
		`
	library {
		name: "qux"
	}
	library {
		name: "hello"
	}
	`), 0644)
	assert.Nil(err)

	head, err := ResolveDepGraph(&headTarget, afs)

	assert.Nil(err)
	assert.Equal(2, len(head.Deps))
}

func TestDiamondDependencyProducesOneNode(t *testing.T) {
	assert := assert.New(t)

	headTarget, _ := target.ParseTarget("//foo:a")
	afs := &afero.Afero{Fs: afero.NewMemMapFs()}
	err := afs.WriteFile("foo/BUILD", []byte(
		`
	library {
		name: "a"
		dependencies: ["//foo:b", "//foo:c"]
	}
	library {
		name: "b"
		dependencies: ["//foo:d"]
	}
	library {
		name: "c"
		dependencies: ["//foo:d"]
	}
	library {
		name: "d"
	}
	`), 0644)
	assert.Nil(err)

	head, err := ResolveDepGraph(&headTarget, afs)

	assert.Nil(err)
	depB := head.Deps[0]
	depC := head.Deps[1]
	depDFromB := depB.Deps[0]
	depDFromC := depC.Deps[0]
	assert.Equal(depDFromB, depDFromC)
}
