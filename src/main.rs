use clap::{Parser, Subcommand};
use include_dir::{include_dir, File};
use std::{env::current_dir, fs, path::PathBuf, str::FromStr, sync::LazyLock};
use regex::Regex;
use std::collections::HashSet;

const COMPONENT_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new("^([A-Za-z]+)/([A-Za-z]+)$").unwrap());

const ALLOWED_FRAMEWORKS: LazyLock<HashSet<&str>> = LazyLock::new(|| {
    let mut set = HashSet::new();
    set.insert("Fusion");
    set.insert("Vide");
    //set.insert("React");

    set
});

const ALLOWED_COMPONENTS: LazyLock<HashSet<&str>> = LazyLock::new(|| {
    let mut set = HashSet::new();
    set.insert("Button");
    //set.insert("Select");
    //set.insert("Input");

    set
});

#[derive(Parser)]
#[command(name = "CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        #[arg(value_enum, required = true)]
        components: Vec<String>,

        #[arg(short, long)]
        path: Option<PathBuf>,

        #[arg(short = 'f', long = "force", action = clap::ArgAction::SetTrue)]
        force: Option<bool>
    },
}

const COMPONENTS: include_dir::Dir<'_> = include_dir!("./components");

fn write_file(path: &str, contents: &[u8], force: bool) -> Result<(), String> {
    // force mode means that files can be overwritten.
    if force {
        // Overwrite the users file (DANGEROUS).
        if fs::metadata(path).is_ok() {
            if fs::write(path, contents).is_err() {
                return Err(format!("Could not create the \"{}\" file!", path))
            }

        // Attempts to create a new file.
        } else {
            File::new(path, contents);
        }

    // We do not want to overwrite the users existing file.
    } else if !fs::metadata(path).is_ok() {
        // Attempts to create a new file.
        if fs::write(path, contents).is_err() {
            return Err(format!("Could not create the \"{}\" file!", path))
        }
    }

    Ok(())
}

fn capitalize_string(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(), // Return an empty string if input is empty
    }
}

fn parse_component_string(component: &str) -> Result<(String, String), String> {
    let captures = COMPONENT_REGEX.captures(&component);

    match captures {
        Some(captures) => {
            if captures.len() != 3 {
                return Err(format!("Skipped {}: \"{}\" Did not meet the expected format of <letters>/<letters>!", component, component))
            }

            let (framework, component) = (
                capitalize_string(captures.get(1).unwrap().as_str()),
                capitalize_string(captures.get(2).unwrap().as_str())
            );

            if !ALLOWED_FRAMEWORKS.contains(framework.as_str()) {
                return Err(format!("Skipped {}: \"{}\" is not a valid framework!", component, framework))
            }
            
            if !ALLOWED_COMPONENTS.contains(component.as_str()) {
                return Err(format!("Skipped {}: \"{}\" is not a valid component!", component, component))
            }

            return Ok((framework, component))
        }

        None => return Err(format!("Skipped {}: \"{}\" Did not meet the expected format of <letters>/<letters>!", component, component))
    }
}

fn add_component(resolved_path: &PathBuf, force: bool, framework: &str, component: &str) {
    let component_name = &component.to_string();
    let component_rel_path = &PathBuf::from_str(component_name).unwrap();

    println!("Adding a {:#?} with the {:#?} framework at the path {:#?}", component_name, &framework, &resolved_path);

    let component_dir = COMPONENTS.get_dir(component_rel_path)
        .expect("Component directory not found!");
    
    let core_file_path: &String = &format!("{}.{}.luau", component_name, framework.to_string());
    let core_file = component_dir.get_file(&format!("{}/{}", component_name, core_file_path))
        .expect(&format!("Stem is missing the \"{}\" file!", core_file_path));

    let logic_file_path = &format!("{}Logic.luau", component_name);
    let logic_file = component_dir.get_file(&format!("{}/{}", component_name, logic_file_path))
        .expect(&format!("Stem is missing the \"{}\" file!", logic_file_path));

    let rsml_file_path = &format!("{}Styles.rsml", component_name);
    let rsml_file = component_dir.get_file(&format!("{}/{}", component_name, rsml_file_path))
        .expect(&format!("Stem is missing the \"{}\" file!", rsml_file_path));


    let users_component_path_buf = &resolved_path.join(component_rel_path);
    fs::create_dir_all(users_component_path_buf)
        .expect(&format!("Could not create the \"{}\" directory!", users_component_path_buf.to_str().unwrap()));


    let users_core_file_path = &users_component_path_buf.join(PathBuf::from_str(core_file_path).unwrap());
    write_file(users_core_file_path.to_str().unwrap(), core_file.contents(), force).unwrap();

    let users_logic_file_path = &users_component_path_buf.join(PathBuf::from_str(logic_file_path).unwrap());
    write_file(users_logic_file_path.to_str().unwrap(), logic_file.contents(), force).unwrap();

    let users_rsml_file_path = &users_component_path_buf.join(PathBuf::from_str(rsml_file_path).unwrap());
    write_file(users_rsml_file_path.to_str().unwrap(), rsml_file.contents(), force).unwrap();
}

fn main() {
    let curr_dir = current_dir().unwrap();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { components, path, force } => {
            let force = match force {
                None => false,
                Some(force) => *force
            };

            let resolved_path = match path {
                Some(path) => {
                    let resolved_path = curr_dir.join(path);
                    if !resolved_path.exists() {
                        panic!("The resolved path does not exist: {:?}", resolved_path);
                    }
                    resolved_path
                },
                None => curr_dir
            };

            for component in components {
                match parse_component_string(&component) {
                    Ok((framework, component)) => add_component(&resolved_path, force, &framework, &component),
                    Err(msg) => eprintln!("{}", msg)
                };
            }

            /*for framework in frameworks {
                for component in components {
                    add_component(&resolved_path, force, framework, component);
                }
            }*/

        }
    }
}