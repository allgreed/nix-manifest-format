mod format_manifest;
mod format_manifest_test;

use std::fs;
use std::path::Path;

use dirs::home_dir;

use format_manifest::format_manifest;

// TODO: Package via nix

fn main()
{
    let home_dir = home_dir().unwrap();
    let manifest_relative_location = Path::new(".nix-profile/manifest.nix");
    let manifest_absolute_location = home_dir.join(manifest_relative_location);

    let manifest_content = fs::read_to_string(manifest_absolute_location)
        .expect("manifest.nix exists in home directory");

    let formatted_manifest = format_manifest(manifest_content);

    print!("{}", formatted_manifest);
}
