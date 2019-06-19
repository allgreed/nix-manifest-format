use rnix::types::*;


pub fn format_manifest(manifest_content: String) -> String
{
    let ast = rnix::parse(&manifest_content);
    let mut buffer = String::with_capacity(manifest_content.capacity());

    buffer.push_str("[\n");

    let package_list = List::cast(ast.root().inner())
        .expect("root element is a list");

    for package in package_list.items()
    {
        let set = Set::cast(package)
            .expect("list element is a set");

        buffer.push_str("  {\n");

        for entry in set.entries()
        {
            buffer.push_str("    ");

            if entry.key().node().first_token().unwrap().text() == "meta"
            {
                buffer.push_str("meta = {};");
            }
            else
            {
                buffer.push_str(&entry.node().to_string());
            }
            buffer.push_str("\n");
        }

        buffer.push_str("  }\n");
    }
    buffer.push_str("]\n");

    buffer
}
