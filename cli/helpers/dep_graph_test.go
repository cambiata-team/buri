package helpers

import (
	"buri/utils/target"
	"fmt"
	"testing"

	"github.com/spf13/afero"
	"github.com/stretchr/testify/assert"
)

func TestErrorsIfBuildFileDoesNotExist(t *testing.T) {
	assert := assert.New(t)

	headTarget, _ := target.ParseTarget("//foo:bar")
	afs := &afero.Afero{Fs: afero.NewMemMapFs()}

	_, err := TopologicallySortDepGraph(&headTarget, afs)

	assert.NotNil(err)
}

func TestErrorsIfBuildFileDoesNotContainTarget(t *testing.T) {
	assert := assert.New(t)

	headTarget, _ := target.ParseTarget("//foo:bar")
	afs := &afero.Afero{Fs: afero.NewMemMapFs()}
	err := afs.WriteFile("foo/BUILD", []byte(""), 0644)
	assert.Nil(err)

	sortedTargets, err := TopologicallySortDepGraph(&headTarget, afs)

	assert.NotNil(err)

	assert.Equal(0, len(sortedTargets))
}

func TestOnlyOneSortedTargetIfTargetHasNoDependencies(t *testing.T) {
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

	sortedTargets, err := TopologicallySortDepGraph(&headTarget, afs)

	assert.Nil(err)
	assert.Equal(1, len(sortedTargets))
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

	_, err = TopologicallySortDepGraph(&headTarget, afs)

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

	_, err = TopologicallySortDepGraph(&headTarget, afs)

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

	sortedTargets, err := TopologicallySortDepGraph(&headTarget, afs)

	assert.Nil(err)

	fmt.Printf("%#v\n", sortedTargets[0].Target.ToString())
	assert.Equal(2, len(sortedTargets))
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

	sortedTargets, err := TopologicallySortDepGraph(&headTarget, afs)

	assert.Nil(err)
	assert.Equal(3, len(sortedTargets))
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

	sortedTargets, err := TopologicallySortDepGraph(&headTarget, afs)

	assert.Nil(err)
	assert.Equal(3, len(sortedTargets))
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

	sortedTargets, err := TopologicallySortDepGraph(&headTarget, afs)

	assert.Nil(err)
	assert.Equal(4, len(sortedTargets))
}

func TestErrorsWithDependencyCycle(t *testing.T) {
	assert := assert.New(t)

	headTarget, _ := target.ParseTarget("//foo:a")
	afs := &afero.Afero{Fs: afero.NewMemMapFs()}
	err := afs.WriteFile("foo/BUILD", []byte(
		`
	library {
		name: "a"
		dependencies: ["//foo:b"]
	}
	library {
		name: "b"
		dependencies: ["//foo:a"]
	}
	`), 0644)
	assert.Nil(err)

	_, err = TopologicallySortDepGraph(&headTarget, afs)

	assert.NotNil(err)
}

func TestTopologicallySortsDependencies(t *testing.T) {
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

	sortedTargets, err := TopologicallySortDepGraph(&headTarget, afs)

	assert.Nil(err)
	assert.Equal(4, len(sortedTargets))

	// first because it does not have any dependencies
	assert.Equal("//foo:d", sortedTargets[0].Target.ToString())
	// the middle two nodes can be in any order
	assert.True("//foo:b" == sortedTargets[1].Target.ToString() || "//foo:c" == sortedTargets[1].Target.ToString())
	assert.True("//foo:b" == sortedTargets[2].Target.ToString() || "//foo:c" == sortedTargets[2].Target.ToString())
	// last because it depends on everything else
	assert.Equal("//foo:a", sortedTargets[3].Target.ToString())
}
