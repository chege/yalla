use std::fs;

use anyhow::{Context, Error};
use toml::Table;

use crate::model::CmdNode;

pub fn table_to_root(root_name: &str, t: &Table) -> CmdNode {
    fn build(name: &str, tbl: &Table) -> CmdNode {
        let description = tbl
            .get("description")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let cmd = tbl
            .get("cmd")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        // Collect child tables into a Vec for the new representation
        let mut children: Vec<CmdNode> = Vec::new();
        for (k, v) in tbl.iter() {
            if let Some(child_tbl) = v.as_table() {
                let child = build(k, child_tbl);
                children.push(child);
            }
        }

        CmdNode {
            name: name.to_string(),
            description,
            cmd,
            children,
        }
    }

    let mut root = build(root_name, t);
    if root.description.is_none()
        && let Some(title) = t.get("title").and_then(|v| v.as_str())
    {
        root.description = Some(title.to_string());
    }
    root
}

pub fn load_toml_table(path: &str) -> Result<Table, Error> {
    let input = fs::read_to_string(path).context(format!("reading {}", path))?;
    let table: Table = toml::from_str(&input).context(format!("parsing {}", path))?;
    Ok(table)
}

#[cfg(test)]
mod tests {
    use crate::model::CmdNode;
    use crate::toml::{load_toml_table, table_to_root};

    #[test]
    fn table_to_root_builds_expected_tree() {
        let table = load_toml_table("tests/fixtures/basic/Yallafile").expect("read fixture");

        let got = table_to_root("yalla", &table);

        let expect = CmdNode {
            name: "yalla".to_string(),
            description: Some("This is a demo top level title".to_string()),
            cmd: None,
            children: vec![
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
                CmdNode {
                    name: "stop".to_string(),
                    description: Some("Stop local services".to_string()),
                    cmd: Some("docker compose down".to_string()),
                    children: vec![],
                },
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
            ],
        };

        assert_eq!(got, expect);
    }
}
