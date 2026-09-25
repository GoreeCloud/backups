#![forbid(unsafe_code)]

use std::process::ExitCode;

use goreecloud_backups_core::{LIFECYCLE, PRODUCT_NAME};
use goreecloud_backups_repository::{RepositoryPersistenceState, persistence_state};

fn main() -> ExitCode {
    let mut arguments = std::env::args().skip(1);

    match arguments.next().as_deref() {
        None | Some("status") => {
            print_status();
            ExitCode::SUCCESS
        }
        Some("--version" | "-V" | "version") => {
            println!("{PRODUCT_NAME} {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        Some(command) => {
            eprintln!("unsupported Development command: {command}");
            eprintln!("available commands: status, version");
            ExitCode::from(2)
        }
    }
}

fn print_status() {
    let persistence = match persistence_state() {
        RepositoryPersistenceState::BlockedUntilFormatAccepted => {
            "blocked-until-format-accepted"
        }
    };

    println!("product: {PRODUCT_NAME}");
    println!("version: {}", env!("CARGO_PKG_VERSION"));
    println!("lifecycle: {LIFECYCLE}");
    println!("backup-engine: not-implemented");
    println!("restore-engine: not-implemented");
    println!("repository-persistence: {persistence}");
    println!("stable-eligible: false");
}
