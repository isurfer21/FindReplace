use regex::Regex;
use serde_json::from_str as from_json_str;
use serde_yaml::from_str as from_yaml_str;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: No option or argument provided.");
        process::exit(0);
    }
    match args[1].as_str() {
        "-h" | "--help" => {
            println!(
                "FindReplace
A tool to find and replace given pattern with substitute inside file.

Syntax:
  findreplace [options]
  findreplace [option] <pattern> <substitute> <file_path>

where,
  pattern      the pattern to search for in the file
  substitute   the text to replace the pattern with or a JSON/YAML/CSON formatted hash-map
  file_path    the path to the file to search and replace in

Options:
  -h --help       Show help menu
  -V --version    Show version info
  -l --log        Show verbose log

Usages:
  findreplace --help
  findreplace --version
  findreplace 'foo' 'bar' file.txt
  findreplace 'hello world' '{{\"hello\":\"hi\",\"world\":\"earth\"}}' file.txt
  findreplace 'hello world' '{{hello: hi, world: earth}}' file.txt
  findreplace 'hello world' 'hello:hi;world:earth' file.txt
  findreplace --log 'hello world' 'hello:hi;world:earth' file.txt

Extras:
  findreplace \"`\\{{[a-zA-Z0-9_-]+`\\}}\" '{{:(;}}:)' file.txt   [PowerShell]
  findreplace '\\{{[a-zA-Z0-9_-]+\\}}' '{{:(;}}:)' file.txt     [Bash]
  findreplace \"\\{{[a-zA-Z0-9_-]+\\}}\" \"{{:(;}}:)\" file.txt    [CMD]
            "
            );
        }
        "-v" | "--version" => {
            println!("FindReplace (v0.1.0)");
        }
        "-l" | "--log" => {
            if args.len() < 4 {
                eprintln!("Error: Not enough arguments provided.");
                process::exit(1);
            }
            let (pattern, substitute, file_path) = (&args[2], &args[3], &args[4]);
            println!(
                "Arguments:\n  Pattern: {}\n  Substitute: {}\n  FilePath: {}",
                pattern, substitute, file_path
            );
            process_file(file_path, substitute, pattern, true);
        }
        _ => {
            if args.len() < 3 {
                eprintln!("Error: Not enough arguments provided.");
                process::exit(1);
            }
            let (pattern, substitute, file_path) = (&args[1], &args[2], &args[3]);
            process_file(file_path, substitute, pattern, false);
        }
    }
}

fn process_file(file_path: &str, substitute: &str, pattern: &str, verbose: bool) {
    let file_loc = Path::new(file_path);
    if file_loc.exists() {
        let revised_contents: String;
        let contents =
            fs::read_to_string(file_path).expect("Something went wrong reading the file");
        if let Ok(map) = from_json_str::<HashMap<String, String>>(substitute) {
            if verbose {
                println!(
                    "Substitute: \n  Format: JSON \n  Interpreted as: {}",
                    serde_json::to_string(&map).unwrap()
                );
            }
            revised_contents = search_and_replace_with_map(&contents, &pattern, &map);
        } else if let Ok(map) = from_yaml_str::<HashMap<String, String>>(substitute) {
            if verbose {
                println!(
                    "Substitute: \n  Format: YAML \n  Interpreted as: {}",
                    serde_json::to_string(&map).unwrap()
                );
            }
            revised_contents = search_and_replace_with_map(&contents, &pattern, &map);
        } else if let Ok(map) = cson_to_hashmap(substitute) {
            if verbose {
                println!(
                    "Substitute: \n  Format: CSON \n  Interpreted as: {}",
                    serde_json::to_string(&map).unwrap()
                );
            }
            revised_contents = search_and_replace_with_map(&contents, &pattern, &map);
        } else {
            if verbose {
                println!("Substitute: \n  Format: String"  );
            }
            revised_contents = search_and_replace(&contents, &pattern, &substitute);
        }
        fs::write(file_path, revised_contents).expect("Unable to write file");
        if verbose {
            println!("Replaced matches!");
        }
    } else {
        println!("Invalid file path!");
    }
}

fn search_and_replace<'a>(contents: &'a str, pattern: &str, substitute: &str) -> String {
    contents.replace(pattern, substitute)
}

fn search_and_replace_with_map<'a>(
    contents: &'a str,
    pattern: &str,
    map: &HashMap<String, String>,
) -> String {
    let mut result = contents.to_string();
    let re = Regex::new(pattern).unwrap();
    for mat in re.find_iter(contents) {
        let mut replaceable_token = mat.as_str().to_string();
        for (key, value) in map {
            replaceable_token = replaceable_token.replace(key, value);
        }
        result = result.replace(mat.as_str(), &replaceable_token);
    }
    result
}

// `cson` refers to `compact-symbol-object-notation`
fn cson_to_hashmap(s: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    if is_valid_cson(s) {
        let mut hm = HashMap::new();
        let separator = s
            .chars()
            .find(|c| [';', ',', '|', '/', '~'].contains(c))
            .unwrap();
        let pairs = s.split(separator);
        for pair in pairs {
            let kv: Vec<&str> = pair.split(':').collect();
            if kv.len() == 2 {
                hm.insert(kv[0].to_string(), kv[1].to_string());
            }
        }
        Ok(hm)
    } else {
        Err("Invalid CSON format".into())
    }
}

fn is_valid_cson(s: &str) -> bool {
    if let Some(separator) = s.chars().find(|c| [';', ',', '|', '/', '~'].contains(c)) {
        let pairs = s.split(separator);
        for pair in pairs {
            let kv: Vec<&str> = pair.split(':').collect();
            if kv.len() != 2 {
                return false;
            }
        }
        true
    } else {
        false
    }
}