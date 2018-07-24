#[macro_use]
extern crate clap;

use std::fs;
use std::env;
use clap::Shell;
use std::path::Path;

fn main() {
	build_shell_completion();
}

/// Build shell completion scripts for all known shells
/// Full list in https://github.com/kbknapp/clap-rs/blob/e9d0562a1dc5dfe731ed7c767e6cee0af08f0cf9/src/app/parser.rs#L123
fn build_shell_completion() {
	let shells = [Shell::Bash, Shell::Fish, Shell::Zsh, Shell::Elvish, Shell::PowerShell];
	for shell in shells.iter() {
    	build_completion(shell);
	}
}

/// Build the shell auto-completion for a given Shell
fn build_completion(shell: &Shell) {
	let yml = load_yaml!("src/cli.yml");

	let outdir = match env::var_os("OUT_DIR") {
        None => return,
        Some(dir) => dir,
    };
    let path = Path::new(&outdir)
    	.parent().unwrap()
    	.parent().unwrap()
    	.parent().unwrap()
    	.join("completion-scripts");

    fs::create_dir(&path).ok();

    let mut app = clap::App::from_yaml(&yml);
    app.gen_completions(
    	"polkadot",
    	*shell,
    	&path);
}
