use std::path::PathBuf;
use std::process::Command;
use dirs;

fn help() {
    eprintln!("Vlugger by {} version {}", env!("CARGO_PKG_AUTHORS"), env!("CARGO_PKG_VERSION"));
    eprintln!("\nvlugger <install | search> <username>/<repo> [--no-vcs]");
    eprintln!("\nOPTIONS:");
    eprintln!("\tinstall : install the specified plugin");
    eprintln!("\tsearch  : search for the specified plugin");
    eprintln!("\nFLAGS:");
    eprintln!("\t--no-vcs : Specifies that the ~/ folder is not version controlled ( a submodule to the plugin will be added without this option)");

    std::process::exit(0);
}
fn match_args(args: Vec<String>)  {
    if &args[3] == "--no-vcs" {
        if &args[1] == "search" {
            search(&args[2]);
        } else if &args[1] == "install" {
            install(&args[2], true);
        } else {
            help();
        } 
    } else {
        if &args[1] == "search" {
            search(&args[2]);
        } else if &args[1] == "install" {
            install(&args[2], false);
        } else {
            help();
        }
    }
}
fn install(package: &str, novcs: bool) {
    if !exists(package) {
        search(package);
        std::process::exit(-1);
    }
    let (user,repo): (&str, &str) = {
        let splited: Vec<&str> = package.split('/').collect();
        if splited.len() != 2 {
            println!("Invalid plugin name");
            std::process::exit(-5);
        }
        (splited[0], splited[1])
    };
   
    let home_dir = dirs::home_dir().unwrap_or(PathBuf::from("~")).into_os_string().into_string().unwrap();

  let output = Command::new("git").arg("clone").arg(&format!("https://github.com/{}/{}.git", user, repo)).arg(&format!("{}/.vim/bundle/{}",home_dir, repo)).output().unwrap();

    if output.status.success() {
        println!("Plugin installed successfully");
    } else {
        println!("An error occured. Please retry later");
    }
    if !novcs {
       let cur_dir = std::env::current_dir().unwrap();
       std::env::set_current_dir(home_dir).unwrap();
       Command::new("git").arg("submodule").arg("add").arg(&format!("https://github.com/{}/{}", user, repo)).arg(&format!(".vim/bundle/{}", repo)).output().unwrap();
       std::env::set_current_dir(cur_dir).unwrap();
    }
}
fn search(package: &str) {
   if exists(package) {
        println!("Plugin `{}` exists", package);
   } else {
        println!("PLugin `{}` does not exists", package);
   }
}
fn exists(package: &str) -> bool {
   let res = Command::new("curl").arg(&format!("https://api.github.com/repos/{}", package)).output().unwrap();
   if std::str::from_utf8(&res.stdout).unwrap().contains("Not Found") {
        return false
   } else {
        return true; 
   }
}
    
fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 2 && args[1] == "-v" {
        println!("{}", env!("CARGO_PKG_VERSION"));
        std::process::exit(0);
    }

    if args.len() == 3 {
        if &args[1] == "search" {
            search(&args[2]);
        } else if &args[1] == "install" {
            install(&args[2], false);
        } else {
            help();
        }       
    } else if args.len() == 4 {
        match_args(args.clone());
    } 

    else {
        help();
    }

}
