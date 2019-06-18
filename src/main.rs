use std::fs;
use std::path::Path;

use dirs::home_dir;

mod format_manifest;

use format_manifest::format_manifest;

// TODO: Unwraps -> expects / sth like that
// TODO: Package via nix

fn main()
{
    let home_dir = home_dir().unwrap();
    let manifest_relative_location = Path::new(".nix-profile/manifest.nix");

    let manifest_content = fs::read_to_string(home_dir.join(manifest_relative_location)).unwrap();

    let formatted_manifest = format_manifest(manifest_content);

    print!("{}", formatted_manifest);
}
