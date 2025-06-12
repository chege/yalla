use std::collections::HashMap;
use std::env;
use std::fs;
use std::process::{exit, Command};

use anyhow::{bail, Context, Result};
use serde::Deserialize;

// New, more flexible struct
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)] // Good practice to avoid typos in the TOML
struct CommandNode {
    // The command to run if this node is executed directly.
    // It's optional because a namespace like [bla] might not have a cmd.
    #[serde(default)]
    cmd: Option<String>,

    // The description for help text. Also optional.
    #[serde(default)]
    description: Option<String>,

    // Nested subcommands. We use a HashMap to hold them.
    // `flatten` is the magic attribute that lets us capture all other keys
    // (like `deploy`, `test`, `tool`) as sub-nodes.
    #[serde(flatten)]
    subcommands: HashMap<String, CommandNode>,
}
// The root of your TOML file is a map from top-level names to CommandNodes
type Yallafile = HashMap<String, CommandNode>;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let raw = fs::read_to_string("Yallafile")?;

    // Deserialize into the new, flexible structure
    let yallafile: Yallafile = toml::from_str(&raw)?;

    if args.len() == 1 {
        // No args, list top-level commands/namespaces
        list_commands("Available commands", &yallafile);
        return Ok(());
    }

    // Start traversing the command tree from the root
    let mut current_level = &yallafile;
    let mut final_node: Option<&CommandNode> = None;

    // Loop through arguments to find the target command
    for i in 1..args.len() {
        let arg = &args[i];
        if let Some(node) = current_level.get(arg) {
            final_node = Some(node);
            current_level = &node.subcommands; // Descend for the next iteration
        } else {
            bail!("Command '{}' not found.", arg);
        }
    }

    // After the loop, decide what to do with the found node
    if let Some(node) = final_node {
        if !node.subcommands.is_empty() && args.len() <= yallafile.keys().count() + 1 {
            // If it has subcommands and the user didn't specify one,
            // either run its direct `cmd` or list the subcommands.
            if let Some(cmd) = &node.cmd {
                return execute_command(cmd);
            } else {
                list_commands(
                    &format!("Available subcommands for '{}'", args.last().unwrap()),
                    &node.subcommands,
                );
                return Ok(());
            }
        }

        if let Some(cmd) = &node.cmd {
            return execute_command(cmd);
        } else {
            bail!(
                "'{}' is a namespace. Please specify a subcommand.",
                args.last().unwrap()
            );
        }
    }

    Ok(())
}

fn list_commands(title: &str, commands: &HashMap<String, CommandNode>) {
    println!("{}:", title);
    for name in commands.keys() {
        println!("  {}", name);
    }
}

fn execute_command(cmd: &str) -> Result<()> {
    let status = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .status()
        .context("Failed to execute command")?;

    if !status.success() {
        exit(status.code().unwrap_or(1));
    }

    Ok(())
}
