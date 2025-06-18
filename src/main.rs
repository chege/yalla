//! # Yalla - A Namespaced Task Runner
mod clap_util;
mod error_util;
mod model;
mod process;
mod toml;

use std::process::exit;

use anyhow::Result;
use clap_util::print_help;

use crate::clap_util::subcommand_path;
use crate::model::build_clap_from_root;
use crate::toml::{load_toml_table, table_to_root};

fn main() -> Result<()> {
    let table = match load_toml_table("Yallafile") {
        Ok(table) => table,
        Err(e) if error_util::not_found(&e) => {
            return Ok(());
        }
        Err(e) => return Err(e),
    };

    let root = table_to_root("yalla", &table);
    let mut clap_root = build_clap_from_root(&root);

    // Parse CLI
    let matches = clap_root.clone().get_matches();
    let path = subcommand_path(&matches);

    if path.is_empty() {
        // Root requested: show top-level help
        print_help(&mut clap_root, &[])?;
        return Ok(());
    }

    match model::find_node(&root, &path) {
        Some(node) => {
            if let Some(cmd) = &node.cmd {
                let status = process::execute(cmd)?;
                exit(process::exit_code(status));
            } else {
                // Namespace-only: show contextual help
                print_help(&mut clap_root, &path)?;
            }
        }
        None => {
            // Shouldn't happen (clap validated), but show the closest help just in case
            print_help(&mut clap_root, &[])?;
        }
    }

    Ok(())
}
