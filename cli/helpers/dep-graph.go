package helpers

import (
	"buri/protos"
	"buri/utils/target"
	"errors"
	"os"

	"google.golang.org/protobuf/encoding/prototext"
)

type DepGraphNode struct {
	Target target.Target
	Files  []string
	Deps   []DepGraphNode
}

func getBuildFileFromTarget(buildFile *protos.BuildFile, currentTarget *target.Target) error {
	filePath := currentTarget.BuildFileLocation()
	fileContents, err := os.ReadFile(filePath)
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
func ResolveDepGraph(thorTarget *target.Target) (DepGraphNode, error) {
	head := DepGraphNode{
		Target: *thorTarget,
	}
	if thorTarget.Name.Kind == target.Recursive {
		return head, errors.New("building recursive targets is not supported yet")
	}
	allNodes := make(map[string]*DepGraphNode)
	stack := []*DepGraphNode{}
	for len(stack) > 0 {
		currentNode := stack[len(stack)-1]
		stack = stack[:len(stack)-1]

		buildFile := protos.BuildFile{}
		err := getBuildFileFromTarget(&buildFile, &currentNode.Target)
		if err != nil {
			return head, err
		}
		for _, library := range buildFile.Library {
			if library.Name == currentNode.Target.Name.Value {
				currentNode.Files = library.Files
				for _, dependency := range library.Dependencies {
					dependencyTarget, err := target.ParseTarget(dependency)
					if err != nil {
						return head, err
					}
					targetString := dependencyTarget.ToString()
					if _, nodeExists := allNodes[targetString]; nodeExists {
						continue
					}
					node := DepGraphNode{
						Target: dependencyTarget,
					}
					stack = append(stack, &node)
					allNodes[targetString] = &node
				}
				break
			}
		}
	}
	return head, nil
}
