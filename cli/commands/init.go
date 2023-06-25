package commands

import (
	"buri/protos"
	"fmt"

	"github.com/spf13/afero"
	"github.com/spf13/cobra"
	prototext "google.golang.org/protobuf/encoding/prototext"
)

var InitializationError error

const workspaceFileName = "WORKSPACE"

func NewInitCommand(fs afero.Fs) cobra.Command {
	command := cobra.Command{
		Use:   "init",
		Short: "Initialize a new Buri workspace",
		Run: func(cmd *cobra.Command, args []string) {
			if InitializationError != nil {
				fmt.Fprint(cmd.OutOrStdout(), InitializationError)
				return
			}
			name, _ := cmd.Flags().GetString("name")
			workspace := &protos.WorkspaceFile{BuriVersion: "nightly"}
			if name != "" {
				workspace.Name = name
			}
			workspaceFile := prototext.Format(workspace)

			if _, err := fs.Stat(workspaceFileName); err == nil {
				fmt.Fprint(cmd.OutOrStdout(), "error: Could not create a new workspace because workspace file already exists")
				return
			}

			f, err := fs.Create(workspaceFileName)

			if err != nil {
				fmt.Fprint(cmd.OutOrStdout(), err.Error())
				return
			}
			defer f.Close()

			_, err2 := f.WriteString(workspaceFile)

			if err2 != nil {
				fmt.Fprint(cmd.OutOrStdout(), err2.Error())
				return
			}

			fmt.Fprint(cmd.OutOrStdout(), "done")
		},
	}

	command.Flags().StringP("name", "n", "", "Name of the project")
	return command
}

func init() {
	initCmd := NewInitCommand(afero.NewOsFs())
	rootCmd.AddCommand(&initCmd)
}
