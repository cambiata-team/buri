package commands

import (
	"buri/cli/build"

	"github.com/spf13/afero"
	"github.com/spf13/cobra"
)

// buildCmd represents the build command
func NewBuildCommand(fs afero.Fs) cobra.Command {
	buildCommand := cobra.Command{
		Use:   "build",
		Short: "Build a target.",
		Args:  cobra.ExactArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			afs := &afero.Afero{Fs: fs}
			return build.BuildTarget(args[0], afs)
		},
	}
	return buildCommand
}
func init() {
	buildCommand := NewBuildCommand(afero.NewOsFs())
	rootCmd.AddCommand(&buildCommand)

}
