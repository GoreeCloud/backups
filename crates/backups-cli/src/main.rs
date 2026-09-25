#![forbid(unsafe_code)]

use std::process::ExitCode;

use goreecloud_backups_core::{LIFECYCLE, PRODUCT_NAME};
use goreecloud_backups_repository::{
    FORMAT_MAJOR, FORMAT_MINOR, FORMAT_PROFILE, RepositoryPersistenceState, persistence_state,
};

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
        RepositoryPersistenceState::BlockedUntilImplementationAccepted => "blocked-until-implementation-accepted",
    };

    println!("product: {PRODUCT_NAME}");
    println!("version: {}", env!("CARGO_PKG_VERSION"));
    println!("lifecycle: {LIFECYCLE}");
    println!("backup-engine: not-implemented");
    println!("restore-engine: not-implemented");
    println!("repository-format: {FORMAT_PROFILE}");
    println!("repository-format-version: {FORMAT_MAJOR}.{FORMAT_MINOR}");
    println!("repository-persistence: {persistence}");
    println!("stable-eligible: false");
}
