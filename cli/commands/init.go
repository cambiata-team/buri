package commands

import (
	"buri/protos"
	"fmt"
	"os"

	"github.com/spf13/cobra"
	prototext "google.golang.org/protobuf/encoding/prototext"
)

var Name string

const workspaceFileName = "WORKSPACE"

// initCmd represents the init command
var initCmd = &cobra.Command{
	Use:   "init",
	Short: "Initialize a new Buri workspace",
	Run: func(cmd *cobra.Command, args []string) {
		workspace := &protos.WorkspaceFile{Name: Name, BuriVersion: "nightly"}
		workspaceFile := prototext.Format(workspace)

		if _, err := os.Stat(workspaceFileName); err == nil {
			fmt.Fprint(cmd.OutOrStdout(), "error: Could not create a new workspace because workspace file already exists")
			return
		}

		f, err := os.Create(workspaceFileName)

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

func init() {
	rootCmd.AddCommand(initCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// initCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// initCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
	initCmd.Flags().StringVarP(&Name, "name", "n", "", "Name of the project")
	initCmd.MarkFlagRequired("name")
}
