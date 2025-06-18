use anyhow::anyhow;
use clap::Command;

pub fn subcommand_path(m: &clap::ArgMatches) -> Vec<String> {
    let mut p = Vec::new();
    let mut cur = m;
    while let Some((name, sub)) = cur.subcommand() {
        p.push(name.to_string());
        cur = sub;
    }
    p
}
pub fn print_help(cmd: &mut Command, path: &[String]) -> anyhow::Result<()> {
    let mut cur = cmd;
    for seg in path {
        cur = cur
            .find_subcommand_mut(seg)
            .ok_or_else(|| anyhow!("No such subcommand: {}", seg))?;
    }
    cur.print_help()?;
    println!();
    Ok(())
}

#[cfg(test)]
mod tests {
    use clap::{Arg, Command};

    use super::*;

    fn demo_cli() -> Command {
        // yalla
        // ├─ stop (leaf)
        // ├─ tools
        // │   ├─ fmt (leaf)
        // │   └─ lint (leaf)
        // └─ script
        //     └─ deploy (leaf)
        let leaf_args = || Arg::new("args").trailing_var_arg(true).num_args(0..);

        let tools = Command::new("tools")
            .subcommand(Command::new("fmt").arg(leaf_args()))
            .subcommand(Command::new("lint").arg(leaf_args()))
            .subcommand_required(true)
            .arg_required_else_help(true);

        let script = Command::new("script")
            // parent itself runnable (has trailing args)
            .arg(leaf_args())
            .subcommand(Command::new("deploy").arg(leaf_args()));

        Command::new("yalla")
            .subcommand(Command::new("stop").arg(leaf_args()))
            .subcommand(tools)
            .subcommand(script)
    }

    #[test]
    fn path_empty_when_no_subcommand() {
        let cmd = demo_cli();
        let m = cmd.clone().get_matches_from(["yalla"]);
        let path = subcommand_path(&m);
        assert!(path.is_empty());
    }

    #[test]
    fn single_level_subcommand() {
        let cmd = demo_cli();
        let m = cmd.clone().get_matches_from(["yalla", "stop"]);
        let path = subcommand_path(&m);
        assert_eq!(path, vec!["stop".to_string()]);
    }

    #[test]
    fn two_levels_subcommand() {
        let cmd = demo_cli();
        let m = cmd.clone().get_matches_from(["yalla", "tools", "fmt"]);
        let path = subcommand_path(&m);
        assert_eq!(path, vec!["tools".to_string(), "fmt".to_string()]);
    }

    #[test]
    fn parent_only_without_child_is_captured() {
        let cmd = demo_cli();
        let m = cmd.clone().get_matches_from(["yalla", "script"]);
        let path = subcommand_path(&m);
        assert_eq!(path, vec!["script".to_string()]);
    }

    #[test]
    fn deep_path_with_trailing_args() {
        let cmd = demo_cli();
        let m = cmd
            .clone()
            .get_matches_from(["yalla", "script", "deploy", "--", "--flag", "value"]);
        let path = subcommand_path(&m);
        assert_eq!(path, vec!["script".to_string(), "deploy".to_string()]);
    }

    #[test]
    fn invalid_subcommand_errors() {
        let cmd = demo_cli();
        let res = cmd.clone().try_get_matches_from(["yalla", "unknown"]);
        assert!(res.is_err());
    }
}
