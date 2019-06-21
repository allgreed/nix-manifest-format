use std::cmp::Ordering;

use rnix::types::*;


#[derive(Eq)]
struct Package
{
    name: String,
    code: String,
}

pub fn format_manifest(manifest_content: String) -> String
{
    let ast = rnix::parse(&manifest_content);
    let mut buffer = String::with_capacity(manifest_content.capacity());

    let package_list = List::cast(ast.root().inner())
        .expect("root element is a list");

    // TODO: Allocate capacity for n packages
    let mut packages = Vec::new();

    // TODO: Redo this functionally
    for package in package_list.items()
    {
        let set = Set::cast(package)
            .expect("list element is a set");

        let pkg = Package::new(set);
        packages.push(pkg);

    }

    packages.sort();

    for pkg in packages
    {
        buffer.push_str(&pkg.code);
    }

    format!("[\n{}]\n", buffer)
}

impl Package
{
    fn new(nix_set: &Set) -> Package
    {
        // TODO: get set range and allocate buffer of that size
        let mut code = String::new();
        let mut name = String::new();

        code.push_str("  {\n");

        for entry in nix_set.entries()
        {
            code.push_str("    ");
            let key = entry.key().node().first_token().unwrap().text();

            if key == "meta"
            {
                code.push_str("meta = {};");
            }
            else
            {
                code.push_str(&entry.node().to_string());
            }

            if key == "name"
            {
                name = entry.value().first_token().unwrap().text().to_string();
            }

            code.push_str("\n");
        }

        code.push_str("  }\n");

        Package{ name, code }
    }
}

impl Ord for Package
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Package
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        Some(self.cmp(other))
    }
}

impl PartialEq for Package
{
    fn eq(&self, other: &Self) -> bool
    {
        self.name == other.name
    }
}
