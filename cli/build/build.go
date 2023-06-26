package build

import (
	"buri/cli/helpers"
	"buri/utils/target"
	"fmt"

	"github.com/spf13/afero"
)

func BuildTarget(rawTarget string, afs *afero.Afero) error {
	parsedTarget, err := target.ParseTarget(rawTarget)
	if err != nil {
		return err
	}
	targetFiles, err := helpers.TopologicallySortDepGraph(parsedTarget, afs)
	if err != nil {
		return err
	}
	for _, targetFile := range targetFiles {
		for _, target := range targetFile.Files {
			fmt.Println(target)
		}
	}
	return err
}
