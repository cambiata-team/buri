package build

import (
	"buri/cli/helpers"
	"buri/utils/target"

	"github.com/spf13/afero"
)

func BuildTarget(rawTarget string, afs *afero.Afero) error {
	parsedTarget, err := target.ParseTarget(rawTarget)
	if err != nil {
		return err
	}
	_, err = helpers.TopologicallySortDepGraph(parsedTarget, afs)
	return err
}
