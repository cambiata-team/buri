package target

import (
	"errors"
	"fmt"
	"regexp"
	"strings"
)

type TargetKind int8

const (
	Specific TargetKind = iota
	Recursive
)

type TargetName struct {
	Kind  TargetKind
	Value string
}

type Target struct {
	IsRelative  bool
	Directories []string
	Name        TargetName
}

func (t Target) BuildFileLocation() string {
	return strings.Join(t.Directories[:], "/") + "/BUILD"
}

func (t Target) ToString() string {
	result := ""
	if !t.IsRelative {
		result += "//"
	}
	result += strings.Join(t.Directories[:], "/")
	if t.Name.Kind == Specific {
		result += ":" + t.Name.Value
	} else {
		result += ":..."
	}
	return result
}

func isValidPart(part string) bool {
	if part == "" {
		return false
	}
	matches, err := regexp.MatchString("^[a-zA-Z0-9_-]+$", part)
	if err != nil {
		return false
	}
	return matches
}

// Parses a Thor target.
func ParseTarget(target string) (Target, error) {
	var isRelative = true
	if target[0:2] == "//" {
		isRelative = false
		target = target[2:]
	}
	if target == "..." {
		return Target{
			IsRelative: isRelative,
			Name: TargetName{
				Kind: Recursive,
			},
		}, nil
	}
	colonParts := strings.Split(target, ":")
	if len(colonParts) > 2 {
		return Target{}, errors.New("too many colons")
	}
	colonPart0 := colonParts[0]
	directories := strings.Split(colonPart0, "/")
	if len(directories) == 1 && directories[0] == "" {
		directories = []string{}
	}
	// loop through directories and validate
	for _, directory := range directories {
		if !isValidPart(directory) {
			return Target{}, fmt.Errorf("invalid directory \"%s\"", directory)
		}
	}
	var targetName = TargetName{Kind: Recursive}
	if len(colonParts) == 2 {
		colonPart1 := colonParts[1]
		if colonPart1 != "..." {
			if !isValidPart(colonPart1) {
				return Target{}, fmt.Errorf("invalid target \"%s\"", target)
			}
			targetName = TargetName{
				Kind:  Specific,
				Value: colonPart1,
			}
		}
	} else {
		if len(directories) == 0 {
			return Target{}, errors.New("expected 1 or more directories")
		}
		finalDirectory := directories[len(directories)-1]
		targetName = TargetName{
			Kind:  Specific,
			Value: finalDirectory,
		}
	}
	return Target{
		IsRelative:  isRelative,
		Directories: directories,
		Name:        targetName,
	}, nil
}
