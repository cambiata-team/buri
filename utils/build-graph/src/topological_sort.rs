use crate::target_files::TargetFiles;
use protobuf::text_format::{parse_from_str, ParseError};
use protos::build::{BuildFile, Library};
use std::collections::HashSet;
use target::{
    parse::{parse_target, TargetParseError},
    Target,
};
use vfs::{VfsError, VfsPath};

type Visited = HashSet<String>;

#[derive(Debug)]
pub enum DependencySortError {
    CyclicDependency(Target),
    VfsError(VfsError),
    BuildFileParseError(ParseError),
    ParseTargetError(TargetParseError),
    TargetNotFoundInBuildFile(Target),
}

fn resolve_build_file(target: &Target, root: &VfsPath) -> Result<BuildFile, DependencySortError> {
    let build_file_path = root
        .join(target.build_file_location())
        .map_err(DependencySortError::VfsError)?;
    let build_file = build_file_path
        .read_to_string()
        .map_err(DependencySortError::VfsError)?;
    parse_from_str::<BuildFile>(&build_file).map_err(DependencySortError::BuildFileParseError)
}

fn topological_sort_helper(
    current_target: Target,
    output: &mut Vec<TargetFiles>,
    visited: &mut Visited,
    temp_visited: &mut Visited,
    root: &VfsPath,
) -> Result<(), DependencySortError> {
    let target_string = current_target.to_string();
    if visited.contains(&target_string) {
        return Ok(());
    }

    if temp_visited.contains(&target_string) {
        return Err(DependencySortError::CyclicDependency(current_target));
    }

    temp_visited.insert(target_string.clone());

    let build_file = resolve_build_file(&current_target, root)?;
    let mut current_library: Option<&Library> = None;

    let target_name_string = current_target.name.to_string();
    for library in &build_file.library {
        if library.name == target_name_string {
            current_library = Some(library);

            for dep in &library.dependencies {
                let dep_target =
                    parse_target(dep).map_err(DependencySortError::ParseTargetError)?;

                topological_sort_helper(dep_target, output, visited, temp_visited, root)?;
            }

            break;
        }
    }

    match current_library {
        Some(library) => {
            temp_visited.remove(&target_string);
            visited.insert(target_string);
            let target_files = TargetFiles {
                target: current_target,
                files: library.files.clone(),
            };
            output.push(target_files);
            Ok(())
        }
        None => Err(DependencySortError::TargetNotFoundInBuildFile(
            current_target,
        )),
    }
}

// TODO: support recursive targets
pub fn topologically_sort_dep_graph(
    head_target: Target,
    root: &VfsPath,
) -> Result<Vec<TargetFiles>, DependencySortError> {
    let mut output = Vec::new();
    let mut visited: Visited = Visited::new();
    let mut temp_visited: Visited = Visited::new();

    topological_sort_helper(
        head_target,
        &mut output,
        &mut visited,
        &mut temp_visited,
        root,
    )?;

    Ok(output)
}

#[cfg(test)]
mod test {
    use super::*;
    use test_utils::create_file::create_test_file;
    use vfs::MemoryFS;

    #[test]
    fn errors_if_build_file_does_not_exist() {
        let root: VfsPath = MemoryFS::new().into();
        let target = parse_target("//foo:bar").unwrap();
        let result = topologically_sort_dep_graph(target, &root);
        assert!(matches!(result, Err(DependencySortError::VfsError(_))));
    }

    #[test]
    fn errors_if_build_file_does_not_contain_target() {
        let root: VfsPath = MemoryFS::new().into();
        let target = parse_target("//foo:bar").unwrap();
        create_test_file(&root, "foo/BUILD", b"");
        let result = topologically_sort_dep_graph(target, &root);
        assert!(matches!(
            result,
            Err(DependencySortError::TargetNotFoundInBuildFile(_))
        ));
    }

    #[test]
    fn only_sorted_one_target_if_target_has_no_dependencies() {
        let root: VfsPath = MemoryFS::new().into();
        let target = parse_target("//foo:bar").unwrap();
        create_test_file(
            &root,
            "foo/BUILD",
            b"
            library {
                name: \"bar\"
            }
            ",
        );
        let result = topologically_sort_dep_graph(target.clone(), &root);
        assert!(matches!(result, Ok(_)));
        let result = result.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].target, target);
    }

    #[test]
    fn errors_if_dependency_build_file_does_not_exist() {
        let root: VfsPath = MemoryFS::new().into();
        let target = parse_target("//foo:bar").unwrap();
        create_test_file(
            &root,
            "foo/BUILD",
            b"
            library {
                name: \"bar\"
                dependencies: \"//baz:qux\"
            }
            ",
        );
        let result = topologically_sort_dep_graph(target, &root);
        assert!(matches!(result, Err(DependencySortError::VfsError(_))));
    }

    #[test]
    fn errors_if_dependency_build_file_does_not_include_target() {
        let root: VfsPath = MemoryFS::new().into();
        let target = parse_target("//foo:bar").unwrap();
        create_test_file(
            &root,
            "foo/BUILD",
            b"
            library {
                name: \"bar\"
                dependencies: \"//fizz:buzz\"
            }
            ",
        );
        create_test_file(&root, "fizz/BUILD", b"");
        let result = topologically_sort_dep_graph(target, &root);
        assert!(matches!(
            result,
            Err(DependencySortError::TargetNotFoundInBuildFile(_))
        ));
    }

    #[test]
    fn adds_dependency_to_build_graph() {
        let root: VfsPath = MemoryFS::new().into();
        let target = parse_target("//foo:bar").unwrap();
        create_test_file(
            &root,
            "foo/BUILD",
            b"
            library {
                name: \"bar\"
                dependencies: \"//fizz:buzz\"
            }
            ",
        );
        create_test_file(
            &root,
            "fizz/BUILD",
            b"
            library {
                name: \"buzz\"
            }
            ",
        );
        let graph = topologically_sort_dep_graph(target, &root).unwrap();
        assert_eq!(graph.len(), 2);
    }

    #[test]
    fn traverses_multiple_build_files() {
        let root: VfsPath = MemoryFS::new().into();
        let target = parse_target("//foo:bar").unwrap();
        create_test_file(
            &root,
            "foo/BUILD",
            b"
            library {
                name: \"bar\"
                dependencies: \"//fizz/buzz:qux\"
                dependencies: \"//hello:world\"
            }
            ",
        );
        create_test_file(
            &root,
            "fizz/buzz/BUILD",
            b"
            library {
                name: \"qux\"
            }
            ",
        );
        create_test_file(
            &root,
            "hello/BUILD",
            b"
            library {
                name: \"world\"
            }
            ",
        );
        let graph = topologically_sort_dep_graph(target, &root).unwrap();
        assert_eq!(graph.len(), 3);
    }

    #[test]
    fn traverses_multiple_targets_in_same_build_file() {
        let root: VfsPath = MemoryFS::new().into();
        let target = parse_target("//foo:bar").unwrap();
        create_test_file(
            &root,
            "foo/BUILD",
            b"
            library {
                name: \"bar\"
                dependencies: \"//fizz/buzz:qux\"
                dependencies: \"//fizz/buzz:world\"
            }
            ",
        );
        create_test_file(
            &root,
            "fizz/buzz/BUILD",
            b"
            library {
                name: \"qux\"
            }
            library {
                name: \"world\"
            }
            ",
        );
        let graph = topologically_sort_dep_graph(target, &root).unwrap();
        assert_eq!(graph.len(), 3);
    }

    #[test]
    fn diamond_dependency_produces_one_node() {
        let root: VfsPath = MemoryFS::new().into();
        let target = parse_target("//foo:a").unwrap();
        create_test_file(
            &root,
            "foo/BUILD",
            b"
            library {
                name: \"a\"
                dependencies: \"//foo:b\"
                dependencies: \"//foo:c\"
            }
            library {
                name: \"b\"
                dependencies: \"//foo:d\"
            }
            library {
                name: \"c\"
                dependencies: \"//foo:d\"
            }
            library {
                name: \"d\"
            }
            ",
        );
        let graph = topologically_sort_dep_graph(target, &root).unwrap();
        // Count would be 5 if we didn't dedupe.
        assert_eq!(graph.len(), 4);
    }

    #[test]
    fn errors_with_dependency_cycle() {
        let root: VfsPath = MemoryFS::new().into();
        let target = parse_target("//foo:a").unwrap();
        create_test_file(
            &root,
            "foo/BUILD",
            b"
            library {
                name: \"a\"
                dependencies: \"//foo:b\"
            }
            library {
                name: \"b\"
                dependencies: \"//foo:a\"
            }
            ",
        );
        let result = topologically_sort_dep_graph(target, &root);
        assert!(matches!(
            result,
            Err(DependencySortError::CyclicDependency(_))
        ));
    }

    #[test]
    fn topologically_sorts_dependencies() {
        let root: VfsPath = MemoryFS::new().into();
        let target = parse_target("//foo:a").unwrap();
        create_test_file(
            &root,
            "foo/BUILD",
            b"
            library {
                name: \"a\"
                dependencies: \"//foo:b\"
                dependencies: \"//foo:c\"
            }
            library {
                name: \"b\"
                dependencies: \"//foo:d\"
            }
            library {
                name: \"c\"
                dependencies: \"//foo:d\"
            }
            library {
                name: \"d\"
            }
            ",
        );
        let graph = topologically_sort_dep_graph(target, &root).unwrap();
        assert_eq!(graph.len(), 4);

        // first because it does not have any dependencies
        assert_eq!(graph[0].target.to_string(), "//foo:d");
        // the middle two nodes can be in any order
        assert!(
            graph[1].target.to_string() == "//foo:b" || graph[1].target.to_string() == "//foo:c"
        );
        assert!(
            graph[2].target.to_string() == "//foo:b" || graph[2].target.to_string() == "//foo:c"
        );
        // last because it depends on everything else
        assert_eq!(graph[3].target.to_string(), "//foo:a");
    }

    #[test]
    fn output_tracks_files() {
        let root: VfsPath = MemoryFS::new().into();
        let target = parse_target("//foo:a").unwrap();
        create_test_file(
            &root,
            "foo/BUILD",
            b"
            library {
                name: \"a\"
                dependencies: \"//foo:b\"
                files: \"a.buri\"
            }
            library {
                name: \"b\"
                files: \"b.buri\"
            }
            ",
        );
        let graph = topologically_sort_dep_graph(target, &root).unwrap();
        assert_eq!(graph.len(), 2);

        assert_eq!(graph[0].files, Vec::from(["b.buri"]));
        assert_eq!(graph[1].files, Vec::from(["a.buri"]));
    }
}
