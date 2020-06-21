use config::Config;
use git::{GitBranch, GitStatus};
use std::env;

mod config;
mod git;

fn main() {
    let config: Config = envy::prefixed("MSHP_").from_env().unwrap();

    let mut items = Vec::new();
    items.push(config.cwd_color.paint(get_working_directory()));

    if !config.git_branch_disable {
        if let Some(git_branch) = GitBranch::from_env() {
            let output = format!("{} {}", config.git_branch_icon, git_branch.name());
            items.push(config.git_branch_color.paint(output));
        }
    }

    if !config.git_status_disable {
        if let Some(git_status) = GitStatus::from_env() {
            let mut output = String::new();
            if git_status.staged > 0 {
                output.push_str(&config.git_staged_icon);
            }
            if git_status.unstaged > 0 {
                output.push_str(&config.git_unstaged_icon);
            }
            if git_status.untracked > 0 {
                output.push_str(&config.git_untracked_icon);
            }
            if git_status.ahead > 0 {
                output.push_str(&config.git_ahead_icon);
            }
            if git_status.behind > 0 {
                output.push_str(&config.git_behind_icon);
            }
            if !output.is_empty() {
                items.push(config.git_status_color.paint(output));
            }
        }
    }

    let euid = unsafe { libc::geteuid() };
    items.push(match euid {
        0 => config.root_indicator_color.paint(config.root_indicator),
        _ => config.user_indicator_color.paint(config.user_indicator),
    });

    let mut output = String::new();
    for item in items {
        output.push_str(&item.bold().to_string());
        output.push(' ');
    }
    println!("{}", output);
}

fn get_working_directory() -> String {
    let mut path = env::var("PWD").expect("failed to get current working directory");
    if let Ok(home_dir) = env::var("HOME") {
        if path.starts_with(&home_dir) {
            path = path.replacen(&home_dir, "~", 1);
        }
    };
    path
}
