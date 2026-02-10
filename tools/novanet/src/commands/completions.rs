//! Shell completion generation.
//!
//! Generates completion scripts for bash, zsh, fish, powershell, and elvish.
//!
//! Usage:
//!   novanet completions bash > ~/.local/share/bash-completion/completions/novanet
//!   novanet completions zsh > ~/.zfunc/_novanet
//!   novanet completions fish > ~/.config/fish/completions/novanet.fish

use clap::CommandFactory;
use clap_complete::{Shell, generate};
use std::io;

/// Generate shell completions and write to stdout.
pub fn run_completions<C: CommandFactory>(shell: Shell) -> crate::Result<()> {
    let mut cmd = C::command();
    let name = cmd.get_name().to_string();
    generate(shell, &mut cmd, name, &mut io::stdout());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_variants() {
        // Just verify the Shell enum values we support
        let shells = [
            Shell::Bash,
            Shell::Zsh,
            Shell::Fish,
            Shell::PowerShell,
            Shell::Elvish,
        ];
        assert_eq!(shells.len(), 5);
    }
}
