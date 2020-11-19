# Vlugger

Vlugger is a simple package manger for Vim plugins.

## Installation

### Prerequisties

This package manager uses [pathogen](https://github.com/tpope/vim-pathogen) and [git](https://git-scm.com).

### Windows

- Go to releases and download the latest executable
**or**
- Run `cargo install vlugger`

### \*Nix

- Go to releases and download the latest executable
**or**
- Download and run installer.sh
**or**
- Run `cargo install vlugger` in `/bin/`

## Getting started

### Install your first plugin

Run `vlugger search <plugin_author>/<plugin_repo>` *Note : The repo has to be on GitHub*

If the repo exists, then run : 

	- If you keep your ~/ version controlled : `vlugger install <plugin_author>/<plugin_repo>`
	- If you don't : `vlugger install <plugin_author>/<plugin_repo> --no-vcs`

### Getting more help

Run `vlugger --help` for help

