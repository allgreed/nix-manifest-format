use std::fs;
use rnix::types::*;

// TODO: Unwraps -> expects / sth like that
// TODO: Accumulate prints and print at the end
// TODO: Expand home in path
// TODO: Add tests

fn main() {
    let file = "/home/allgreed/.nix-profile/manifest.nix";

    let content = match fs::read_to_string(file) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("error reading file: {}", err);
            return;
        }
    };

    let ast = rnix::parse(&content);
    let package_list = List::cast(ast.root().inner()).unwrap();

    println!("[");

    for package in package_list.items() {
        let set = Set::cast(package).unwrap();
        println!("  {{");

        for entry in set.entries() {
            if entry.key().node().first_token().unwrap().text() == "meta" {
                println!("    meta = {{}};");
            }
            else {
                println!("    {}", entry.node());
            }
        }

        println!("  }}");
    }

    println!("]");
}
