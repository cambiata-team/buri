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
	kind  TargetKind
	value string
}

type ThorTarget struct {
	isRelative  bool
	directories []string
	name        TargetName
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
func parseTarget(target string) (ThorTarget, error) {
	var isRelative = true
	if target[0:2] == "//" {
		isRelative = false
		target = target[2:]
	}
	if target == "..." {
		return ThorTarget{
			isRelative: isRelative,
			name: TargetName{
				kind: Recursive,
			},
		}, nil
	}
	colonParts := strings.Split(target, ":")
	if len(colonParts) > 2 {
		return ThorTarget{}, errors.New("too many colons")
	}
	colonPart0 := colonParts[0]
	directories := strings.Split(colonPart0, "/")
	if len(directories) == 1 && directories[0] == "" {
		directories = []string{}
	}
	// loop through directories and validate
	for _, directory := range directories {
		if !isValidPart(directory) {
			return ThorTarget{}, fmt.Errorf("invalid directory \"%s\"", directory)
		}
	}
	var targetName = TargetName{kind: Recursive}
	if len(colonParts) == 2 {
		colonPart1 := colonParts[1]
		if colonPart1 != "..." {
			if !isValidPart(colonPart1) {
				return ThorTarget{}, fmt.Errorf("invalid target \"%s\"", target)
			}
			targetName = TargetName{
				kind:  Specific,
				value: colonPart1,
			}
		}
	} else {
		if len(directories) == 0 {
			return ThorTarget{}, errors.New("expected 1 or more directories")
		}
		finalDirectory := directories[len(directories)-1]
		targetName = TargetName{
			kind:  Specific,
			value: finalDirectory,
		}
	}
	return ThorTarget{
		isRelative:  isRelative,
		directories: directories,
		name:        targetName,
	}, nil
}
