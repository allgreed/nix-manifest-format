use std::fs;
use std::path::Path;

use dirs::home_dir;
use rnix::types::*;

// TODO: Add tests
// TODO: Accumulate prints and print at the end
// TODO: Unwraps -> expects / sth like that
// TODO: Package via nix

fn main() {

    let home_dir = home_dir().unwrap();
    let manifest_relative_location = Path::new(".nix-profile/manifest.nix");

    let content = fs::read_to_string(home_dir.join(manifest_relative_location)).unwrap();
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
