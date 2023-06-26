package helpers

import (
	"buri/protos"
	"buri/utils/target"
	"errors"

	"github.com/spf13/afero"
	"google.golang.org/protobuf/encoding/prototext"
)

type DepGraphNode struct {
	Target target.Target
	Files  []string
	Deps   []*DepGraphNode
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

// TODO: error on circular dependencies
func ResolveDepGraph(headTarget *target.Target, afs *afero.Afero) (DepGraphNode, error) {
	head := DepGraphNode{
		Target: *headTarget,
	}
	if headTarget.Name.Kind == target.Recursive {
		return head, errors.New("building recursive targets is not supported yet")
	}
	headString := headTarget.ToString()
	allNodes := make(map[string]*DepGraphNode)
	allNodes[headString] = &head
	stack := []*DepGraphNode{&head}
	for len(stack) > 0 {
		currentNode := stack[len(stack)-1]
		stack = stack[:len(stack)-1]

		buildFile := protos.BuildFile{}
		err := getBuildFileFromTarget(&buildFile, &currentNode.Target, afs)
		if err != nil {
			return head, err
		}
		var isTargetInBuildFile = false
		for _, library := range buildFile.Library {
			if library.Name == currentNode.Target.Name.Value {
				currentNode.Files = library.Files
				for _, dependency := range library.Dependencies {
					dependencyTarget, err := target.ParseTarget(dependency)
					if err != nil {
						return head, err
					}
					targetString := dependencyTarget.ToString()
					if node, nodeExists := allNodes[targetString]; nodeExists {
						currentNode.Deps = append(currentNode.Deps, node)
						continue
					}
					node := DepGraphNode{
						Target: dependencyTarget,
					}
					stack = append(stack, &node)
					allNodes[targetString] = &node
					currentNode.Deps = append(currentNode.Deps, &node)
				}
				isTargetInBuildFile = true
				break
			}
		}
		if !isTargetInBuildFile {
			return head, errors.New("target not found in BUILD file")
		}
	}
	return head, nil
}
