use std::path::PathBuf;
use std::process::Command;
use dirs;
use std::fs;

fn help() {
    eprintln!("Vlugger by {} version {}", env!("CARGO_PKG_AUTHORS"), env!("CARGO_PKG_VERSION"));
    eprintln!("\nvlugger <install | search | update | uninstall> <username>/<repo> [--no-vcs]");
    eprintln!("\nOPTIONS:");
    eprintln!("\tupdate    : update all the plugins in ~/.vim/bundle");
    eprintln!("\tinstall   : install the specified plugin");
    eprintln!("\tsearch    : search for the specified plugin");
    eprintln!("\tuninstall : uninstall the specified plugin");
    eprintln!("\nFLAGS:");
    eprintln!("\t--no-vcs  : Specifies that the ~/ folder is not version controlled ( a submodule to the plugin will be added without this option)");

    std::process::exit(0);
}

fn update() {
    let home_dir = dirs::home_dir().unwrap_or(PathBuf::from("~")).into_os_string().into_string().unwrap();
    let dir = match fs::read_dir(&format!("{}/.vim/bundle", home_dir)) {
        Ok(d) => d,
        Err(_e) => {
            eprintln!("Cannot find directory {}/.vim/bundle/", home_dir);
            std::process::exit(-8);
        }
    }; 

    for entry in dir {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Failed to read dir");
                eprintln!("Debug info : {}", e);
                std::process::exit(-7);
            }
        };
        let path = entry.path();

        if path.is_dir() {
            std::env::set_current_dir(path).unwrap();
            Command::new("git").arg("pull").status().unwrap();
        }
    }
}

fn match_args(args: Vec<String>)  {
    if &args[3] == "--no-vcs" {
        if &args[1] == "search" {
            search(&args[2]);
        } else if &args[1] == "install" {
            install(&args[2], true);
        }  else if &args[1] == "uninstall" {
            uninstall(&args[2]);
        } else {
            help();
        } 
    } else {
        if &args[1] == "search" {
            search(&args[2]);
        } else if &args[1] == "install" {
            install(&args[2], false);
        } else if &args[1] == "uninstall" {
            uninstall(&args[2]);
        } else {
            help();
        }
    }
}

fn uninstall(package: &str) {
    let home_dir = dirs::home_dir().unwrap_or(PathBuf::from("~")).into_os_string().into_string().unwrap();
    let dir = match fs::read_dir(&format!("{}/.vim/bundle", home_dir)) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Cannot find directory {}/.vim/bundle/", home_dir);
            eprintln!("Debug info : {}", e);
            std::process::exit(-8);
        }
    }; 
    let mut found = false;
    for entry in dir {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Failed to read Dir");
                eprintln!("Debug info : {}", e);
                std::process::exit(-7);
            }
        };
        let path = entry.path();
        if path == std::path::Path::new(&format!("{}/.vim/bundle/{}", home_dir, package)) {
            match fs::remove_dir_all(entry.path()) {
                Ok(()) => {
                    found = true;
                }
                Err(e) => {
                    eprintln!("Failed to remove folder");
                    eprintln!("Debug info : {}", e);
                    std::process::exit(-8); 
                }
            }
        }

    }

    if found {
        println!("Successfully uninstalled `{}`", package);
    } else {
        eprintln!("Package `{}` not found", package);
        std::process::exit(-9);
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
        let langs = Command::new("curl").arg(&format!("https://api.github.com/repos/{}/languages", package)).output().unwrap();
        if std::str::from_utf8(&langs.stdout).unwrap().contains("Vim") {
            return true;
        } else {
            return false;
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 2{
        if &args[1] == "-v" {
            println!("{}", env!("CARGO_PKG_VERSION"));
            std::process::exit(0);
        } else if &args[1] == "update" {
            update();
            std::process::exit(0);
        }
    }

    if args.len() == 3 {
        if &args[1] == "search" {
            search(&args[2]);
        } else if &args[1] == "install" {
            install(&args[2], false);
        } else if &args[1] == "uninstall" {
            uninstall(&args[2]);    
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
