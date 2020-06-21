# mshp

[![Build](https://img.shields.io/github/workflow/status/yuqio/mshp/CI)](https://github.com/yuqio/mshp/actions)
[![Crate](https://img.shields.io/crates/v/mshp)](https://crates.io/crates/mshp)
[![Version](https://img.shields.io/github/v/release/yuqio/mshp?color=orange)](https://github.com/yuqio/mshp/releases)
[![License](https://img.shields.io/crates/l/mshp?color=yellow)](https://github.com/yuqio/mshp#license)

<img width="50%" align="right" src="screenshot.png" />

**A minimal and fast shell prompt written in Rust.**

- **Fast:** 🚀
- **Universal:** Works on any shell, on any operating system
- **Easy-to-hack:** Very easy to add a feature or change a behaviour due to the minimal codebase

## Installation

### Building from source

1. Install and setup [Rust](https://www.rust-lang.org)
2. Run `cargo install mshp`

### Pre-compiled binary

1. Download a binary from the [releases page](https://github.com/yuqio/mshp/releases)
2. Move the binary to your `PATH`

## Usage

### Bash, Dash, Ksh, etc.

On most POSIX compatible shells you can add the following line to your init file (e.g. `.bashrc`):

```sh
PS1="$(mshp)"
```

### Zsh

Add the following to your `.zshrc` file:

```zsh
precmd() {
    PS1="$(mshp)"
}
```

### Ion

Add the following to your `.config/ion/initrc` file:

```ion
fn PROMPT
    echo -n "$(mshp)"
end
```

## Configuration

`mshp` can be configured via environment variables.

### Variable types

- **`Color`**: Takes one of the following as value:

    - `default` or empty (sets the color to the default foreground color defined by the terminal)
    - A written out ANSII color (`black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`)
    - A ANSII number (e.g. `1` for red)
    - A hex color beginning with `#` (e.g. `#00F` or `#0000FF` for blue)

- **`Boolean`**: Takes either `0` (deactivate setting) or `1` (activate setting) as value

- **`String`**: Takes any value

### Available environment variables

- **`MSHP_CWD_COLOR`**

    Sets the foreground color used to display the current working directory.

    Type: `Color`, Default: `blue`

- **`MSHP_GIT_BRANCH_ICON`**

    Sets the icon that is displayed next to the git branch.

    Type: `String`, Default: ``

- **`MSHP_GIT_BRANCH_COLOR`**

    Sets the foreground color used to display the git branch and the icon.

    Type: `Color`, Default: `cyan`

- **`MSHP_GIT_BRANCH_DISABLE`**

    Disables the git branch and icon.

    Type: `Boolean`, Default: `0`

- **`MSHP_GIT_STAGED_ICON`**

    Sets the icon that is used to indicate uncommited and staged changes in the git repo.

    Type: `String`, Default: `+`

- **`MSHP_GIT_UNSTAGED_ICON`**

    Sets the icon that is used to indicate uncommited and unstaged changes in the git repo.

    Type: `String`, Default: `!`

- **`MSHP_GIT_UNTRACKED_ICON`**

    Sets the icon that is used to indicate untracked files in the git repo.

    Type: `String`, Default: `!`

- **`MSHP_GIT_AHEAD_ICON`**

    Sets the icon that is used to indicate that your local branch is ahead of the upstream branch.

    Type: `String`, Default: `↥`

- **`MSHP_GIT_BEHIND_ICON`**

    Sets the icon that is used to indicate that your local branch is behind the upstream branch.

    Type: `String`, Default: `↧`

- **`MSHP_GIT_STATUS_COLOR`**

    Sets the color for the above mentioned icons.

    Type: `Color`, Default: `cyan`

- **`MSHP_GIT_STATUS_DISABLE`**

    Disables the above mentioned icons.

    Type: `Boolean`, Default: `0`

- **`MSHP_USER_INDICATOR`**

    Sets the icon that is displayed at the end of the prompt to indicate that the current user is
    not the root user.

    Type: `String`, Default: `$`

- **`MSHP_USER_INDICATOR_COLOR`**

    Sets the color for the user indicator icon.

    Type: `Color`, Default: `green`

- **`MSHP_ROOT_INDICATOR`**

    Sets the icon that is displayed at the end of the prompt to indicate that the current user is
    the root user.

    Type: `String`, Default: `#`

- **`MSHP_ROOT_INDICATOR_COLOR`**

    Sets the color for the root indicator icon.

    Type: `Color`, Default: `green`

## License

Licensed under either of [Apache License, Version 2.0] or [MIT License] at your
option.

[Apache License, Version 2.0]: https://github.com/yuqio/parg/blob/master/LICENSE-APACHE
[MIT License]: https://github.com/yuqio/parg/blob/master/LICENSE-MIT

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
