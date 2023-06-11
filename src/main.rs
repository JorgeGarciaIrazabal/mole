use std::process::Command;
use serde_json::{Result, Value};
use serde::{Serialize, Deserialize};

// serializible struct for json output that has a dependency name, and list of strings as dependencies constrains
#[derive(Serialize, Deserialize)]
struct Dependency {
    name: String,
    constrains: Vec<String>,
}


// function that takes a package name and a version and returns its dependencies
fn get_dependencies(package_name: &str, package_version: &str) -> Result<Vec<Dependency>> {
    // execute command line interface and parse json output
    let output = Command::new("poetry")
        .current_dir("/home/jorge/code/mole/deps_finder")
        .arg("run")
        .arg("python")
        .arg("find_dependencies.py") 
        .arg("find-dependencies")
        .arg(package_name)
        .arg(package_version)
        .output()
        .expect("failed to execute process");
    let stdout = String::from_utf8(output.stdout).unwrap();
    // parse output into json object
    let raw_deps: Vec<String> = serde_json::from_str(&stdout).unwrap();
    // parse raw_deps into Dependency structs
    let mut dependencies: Vec<Dependency> = Vec::new();
    for raw_dep in raw_deps {
        // parse raw_dep into Dependency struct using regex
        // input examples: "pytz", "pytz (>=2019.1)", "asgiref (~=3.2)", "argon2-cffi (>=16.1.0)",
        let re = regex::Regex::new(r"([a-zA-Z0-9\-]+)\s?(.*)").unwrap();
        
        let captures = re.captures(&raw_dep).unwrap();
        let name = captures.get(1).unwrap().as_str().to_string();
        let mut constrains: Vec<String> = Vec::new();
        if let Some(constrains_str) = captures.get(2) {
            let constrains_str = constrains_str.as_str().split(",").collect::<Vec<&str>>();
            // split constrains by comma
            for cap in constrains_str {
                constrains.push(cap.trim().to_string());
            }
        }

        // create Dependency struct
        let dependency = Dependency {
            name: name,
            constrains: constrains,
        };
        dependencies.push(dependency);
    }
    return Ok(dependencies);
}


fn main() {
    // get dependencies of a package
    let dependencies = get_dependencies("pydantic", "1.10").unwrap();
    // print dependencies
    for dependency in dependencies {
        println!("{}: {:?}", dependency.name, dependency.constrains);
    }
}