pub fn format_package_list(mut package_list: String) -> String {
    package_list = package_list.lines().map(|line| format!("\t\"{}\",\n", line)).collect();
    package_list = format!("install = [\n{}]", package_list);

    package_list
}
