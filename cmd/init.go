package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
)

var Name string

// initCmd represents the init command
var initCmd = &cobra.Command{
	Use:   "init",
	Short: "Initialize a new Buri workspace",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("init called")
		fmt.Println(Name)
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
