fn help() {
    eprintln!("Plugger by {} version {}", env!("CARGO_PKG_AUTHORS"), env!("CARGO_PKG_VERSION"));
    eprintln!("\nplugger <install | search> <provider>:<username>/<repo> [--no-vcs]");
    eprintln!("\nOPTIONS:");
    eprintln!("\tinstall : install the specified plugin");
    eprintln!("\tsearch  : search for the specified plugin");
    eprintln!("\nFLAGS:");
    eprintln!("\t--no-vcs : Specifies that the ~/.vim/ folder is not version controlled");

    std::process::exit(0);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        help();
    }

}
