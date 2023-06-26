package helpers

import (
	"buri/protos"
	"buri/utils/target"
	"errors"
	"fmt"

	"github.com/spf13/afero"
	"google.golang.org/protobuf/encoding/prototext"
)

type TargetFiles struct {
	Target *target.Target
	Files  *[]string
}

func getBuildFileFromTarget(buildFile *protos.BuildFile, currentTarget *target.Target, afs *afero.Afero) error {
	filePath := currentTarget.BuildFileLocation()
	fileContents, err := afs.ReadFile(filePath)
	if err != nil {
		return err
	}
	err2 := prototext.Unmarshal(fileContents, buildFile)
	if err2 != nil {
		return err2
	}
	return nil
}

func topologicalSortHelper(
	currentTarget *target.Target,
	visited *map[string]struct{},
	tempVisited *map[string]struct{},
	sortedTargets *[]*TargetFiles,
	afs *afero.Afero,
) error {
	if _, ok := (*visited)[currentTarget.ToString()]; ok {
		return nil
	}
	if _, ok := (*tempVisited)[currentTarget.ToString()]; ok {
		return errors.New("circular dependency detected")
	}

	(*tempVisited)[currentTarget.ToString()] = struct{}{}

	// do stuff
	buildFile := &protos.BuildFile{}
	err := getBuildFileFromTarget(buildFile, currentTarget, afs)
	if err != nil {
		return err
	}
	if buildFile.Library == nil {
		return fmt.Errorf("build file '%s' does not contain a library", currentTarget.BuildFileLocation())
	}
	foundLibraryInBuildFile := false
	for _, library := range buildFile.Library {
		if library.Name == currentTarget.Name.Value {
			foundLibraryInBuildFile = true
			for _, dependency := range library.Dependencies {
				dependencyTarget, err := target.ParseTarget(dependency)
				if err != nil {
					return err
				}
				err = topologicalSortHelper(&dependencyTarget, visited, tempVisited, sortedTargets, afs)
				if err != nil {
					return err
				}
			}
			break
		}
	}
	if !foundLibraryInBuildFile {
		return fmt.Errorf("library '%s' not found in build file '%s'", currentTarget.Name.Value, currentTarget.BuildFileLocation())
	}

	// cleanup
	delete(*tempVisited, currentTarget.ToString())
	(*visited)[currentTarget.ToString()] = struct{}{}
	*sortedTargets = append(*sortedTargets, &TargetFiles{currentTarget, nil})
	return nil
}

func TopologicallySortDepGraph(headTarget *target.Target, afs *afero.Afero) ([]*TargetFiles, error) {
	output := []*TargetFiles{}
	if headTarget.Name.Kind == target.Recursive {
		return output, errors.New("building recursive targets is not supported yet")
	}

	visited := make(map[string]struct{})
	tempVisited := make(map[string]struct{})
	error := topologicalSortHelper(headTarget, &visited, &tempVisited, &output, afs)

	if error != nil {
		return output, error
	}
	return output, nil
}
