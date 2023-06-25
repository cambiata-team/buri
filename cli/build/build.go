package build

import (
	"buri/cli/helpers"
	"buri/utils/target"
)

func BuildTarget(rawTarget string) error {
	parsedTarget, err := target.ParseTarget(rawTarget)
	if err != nil {
		return err
	}
	_, err = helpers.ResolveDepGraph(&parsedTarget)
	return err
}
