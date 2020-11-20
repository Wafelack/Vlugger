<div align="center">
	
# Vlugger

Vlugger is a simple plugin manager for Vim.
	
![Crates.io](https://img.shields.io/crates/d/vlugger?label=Downloads%20%28crates.io%29)
![GitHub Repo stars](https://img.shields.io/github/stars/Wafelack/vlugger?color=sucess&label=Stars&logo=Github)
		
</div>

## Installation

### Prerequisties

This package manager uses [pathogen](https://github.com/tpope/vim-pathogen) and [git](https://git-scm.com).

### Windows

You can either :

- Go to releases and download the latest executable
- Run `cargo install vlugger`

### \*Nix

You can either :
 
- Go to releases and download the latest executable
- Download and run installer.sh
- Run `cargo install vlugger` in `/bin/`

## Getting started

### Install your first plugin

Run `vlugger search <plugin_author>/<plugin_repo>` *Note : The repo has to be on GitHub*

If the repo exists, then run : 

- If you keep your `~/` version controlled : `vlugger install <plugin_author>/<plugin_repo>`
- If you don't : `vlugger install <plugin_author>/<plugin_repo> --no-vcs`

### Update your plugins

Run `vlugger update` to update all the plugins located in `~/.vim/bundle/`.

### Getting more help

Run `vlugger --help` for help

