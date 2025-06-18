/// Internal command tree model independent of clap
use clap::{Arg, Command};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmdNode {
    pub name: String,
    pub description: Option<String>,
    pub cmd: Option<String>,
    pub children: Vec<CmdNode>,
}

// Minimal string interner to satisfy clap's `'static` requirement for names
fn safe_intern(s: &str) -> &'static str {
    use std::collections::HashMap;
    use std::sync::{LazyLock, Mutex};
    static INTERN: LazyLock<Mutex<HashMap<String, &'static str>>> =
        LazyLock::new(|| Mutex::new(HashMap::new()));
    let mut map = INTERN.lock().unwrap();
    if let Some(&r) = map.get(s) {
        return r;
    }
    let leaked: &'static str = Box::leak(s.to_string().into_boxed_str());
    map.insert(s.to_string(), leaked);
    leaked
}

pub fn build_clap_from_root(root: &CmdNode) -> Command {
    fn to_cmd(n: &CmdNode) -> Command {
        let mut c = Command::new(safe_intern(&n.name));
        if let Some(d) = &n.description {
            c = c.about(d.clone());
        }

        // Stable order for help/tests
        let mut kids = n.children.clone();
        kids.sort_by(|a, b| a.name.cmp(&b.name));
        for ch in &kids {
            c = c.subcommand(to_cmd(ch));
        }

        if n.cmd.is_some() {
            c = c.arg(Arg::new("args").trailing_var_arg(true).num_args(0..));
        } else if !kids.is_empty() {
            c = c.subcommand_required(true).arg_required_else_help(true);
        }
        c
    }

    to_cmd(root)
}

pub fn find_node<'a>(root: &'a CmdNode, path: &[String]) -> Option<&'a CmdNode> {
    let mut cur = root;
    for seg in path {
        cur = cur.children.iter().find(|c| &c.name == seg)?;
    }
    Some(cur)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Shared fixture: full tree with all permutations
    fn fixture_root() -> CmdNode {
        CmdNode {
            name: "yalla".to_string(),
            description: Some("This is a demo top level title".to_string()),
            cmd: None,
            children: vec![
                // A) Top-level leaf
                CmdNode {
                    name: "stop".to_string(),
                    description: Some("Stop local services".to_string()),
                    cmd: Some("docker compose down".to_string()),
                    children: vec![],
                },
                // B) Explicit parent + children
                CmdNode {
                    name: "tools".to_string(),
                    description: Some("Developer tooling".to_string()),
                    cmd: None,
                    children: vec![
                        CmdNode {
                            name: "fmt".to_string(),
                            description: None,
                            cmd: Some("cargo fmt".to_string()),
                            children: vec![],
                        },
                        CmdNode {
                            name: "lint".to_string(),
                            description: None,
                            cmd: Some(
                                "cargo clippy --all-targets --all-features -- -D warnings"
                                    .to_string(),
                            ),
                            children: vec![],
                        },
                        CmdNode {
                            name: "ls".to_string(),
                            description: None,
                            cmd: Some("ls -1".to_string()),
                            children: vec![],
                        },
                    ],
                },
                // C) Parent with cmd AND children
                CmdNode {
                    name: "script".to_string(),
                    description: Some("Project scripts (also runnable)".to_string()),
                    cmd: Some("bash ./scripts/run.sh".to_string()),
                    children: vec![
                        CmdNode {
                            name: "deploy".to_string(),
                            description: None,
                            cmd: Some("bash ./scripts/deploy.sh --env=prod".to_string()),
                            children: vec![],
                        },
                        CmdNode {
                            name: "test".to_string(),
                            description: None,
                            cmd: Some("cargo test --all --all-features".to_string()),
                            children: vec![],
                        },
                    ],
                },
                // D) Implicit parent (single child)
                CmdNode {
                    name: "db".to_string(),
                    description: None,
                    cmd: None,
                    children: vec![CmdNode {
                        name: "migrate".to_string(),
                        description: None,
                        cmd: Some("diesel migration run".to_string()),
                        children: vec![],
                    }],
                },
                // E) Implicit parent (multiple children)
                CmdNode {
                    name: "ci".to_string(),
                    description: None,
                    cmd: None,
                    children: vec![
                        CmdNode {
                            name: "build".to_string(),
                            description: None,
                            cmd: Some("cargo build --release".to_string()),
                            children: vec![],
                        },
                        CmdNode {
                            name: "test".to_string(),
                            description: None,
                            cmd: Some("cargo test --workspace".to_string()),
                            children: vec![],
                        },
                    ],
                },
                // F) Deep implicit parents
                CmdNode {
                    name: "kube".to_string(),
                    description: None,
                    cmd: None,
                    children: vec![CmdNode {
                        name: "dev".to_string(),
                        description: None,
                        cmd: None,
                        children: vec![CmdNode {
                            name: "apply".to_string(),
                            description: None,
                            cmd: Some("kubectl apply -k k8s/overlays/dev".to_string()),
                            children: vec![],
                        }],
                    }],
                },
                // G) Another realistic leaf under a different namespace
                CmdNode {
                    name: "git".to_string(),
                    description: None,
                    cmd: None,
                    children: vec![CmdNode {
                        name: "status".to_string(),
                        description: None,
                        cmd: Some("git rev-parse --is-inside-work-tree".to_string()),
                        children: vec![],
                    }],
                },
            ],
        }
    }

    // Helper for extracting the subcommand path from clap matches
    fn subcommand_path(m: &clap::ArgMatches) -> Vec<String> {
        let mut p = Vec::new();
        let mut cur = m;
        while let Some((name, sub)) = cur.subcommand() {
            p.push(name.to_string());
            cur = sub;
        }
        p
    }

    mod clap_build {
        use super::*;

        #[test]
        fn runnable_leaf_accepts_trailing_args() {
            let cmd = build_clap_from_root(&fixture_root());
            let m = cmd.clone().get_matches_from(["yalla", "stop", "--", "-v"]);
            assert_eq!(subcommand_path(&m), vec!["stop".to_string()]);
        }

        #[test]
        fn pure_namespace_requires_subcommand() {
            let cmd = build_clap_from_root(&fixture_root());
            let res = cmd.clone().try_get_matches_from(["yalla", "tools"]);
            assert!(res.is_err(), "tools without child should error");
        }

        #[test]
        fn runnable_parent_and_child_paths() {
            let cmd = build_clap_from_root(&fixture_root());

            // Parent runnable without child
            let m1 = cmd.clone().get_matches_from(["yalla", "script"]);
            assert_eq!(subcommand_path(&m1), vec!["script".to_string()]);

            // Child runnable
            let m2 = cmd.clone().get_matches_from(["yalla", "script", "deploy"]);
            assert_eq!(
                subcommand_path(&m2),
                vec!["script".to_string(), "deploy".to_string()]
            );
        }

        #[test]
        fn deep_nesting_path() {
            let cmd = build_clap_from_root(&fixture_root());
            let m = cmd
                .clone()
                .get_matches_from(["yalla", "kube", "dev", "apply"]);
            assert_eq!(
                subcommand_path(&m),
                vec!["kube".to_string(), "dev".to_string(), "apply".to_string()]
            );
        }

        #[test]
        fn invalid_subcommand_errors() {
            let cmd = build_clap_from_root(&fixture_root());
            let res = cmd.clone().try_get_matches_from(["yalla", "unknown"]);
            assert!(res.is_err());
        }
    }
}
