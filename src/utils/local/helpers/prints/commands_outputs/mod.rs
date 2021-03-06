use crate::utils::interplanetary::multiformats::DEFAULT_MULTIBASE;
use cid::Cid;
use console::style;

/*
Success messages
*/

/// Print CREATE method success message.
pub fn print_create_success(key: &str) {
    println!(
        "{}",
        style(format!("new object created: {}", style(key).bold())).green()
    )
}

/// Print UPDATE method success message.
pub fn print_update_success(key: &str) {
    println!(
        "{}",
        style(format!("object updated: {}", style(key).bold())).green()
    )
}

/// Print DELETE method success message.
pub fn print_delete_success(key: &str) {
    println!(
        "{}",
        style(format!("object deleted: {}", style(key).bold())).green()
    )
}

/// Print success message for methods checking the health of the transformation pipeline currently
/// in the local area.
pub fn print_local_pipeline_health_success() {
    println!(
        "{}",
        style("current local project holds a healthy transformation pipeline").green()
    )
}

/// Print success message for methods checking the ability to parse the pipeline currently in the
/// interplanetary area
pub fn print_interplanetary_health_success() {
    println!(
        "{}",
        style("interplanetary area holds a healthy transformation pipeline").green()
    )
}

/// Print project EXPORT success message.
pub fn print_project_export_success(cid: &Cid) {
    let cid_str = cid
        .to_string_of_base(DEFAULT_MULTIBASE)
        .unwrap_or("".to_string());
    println!(
        "{}",
        style(format!(
            "project exported with pipeline cid: {}",
            style(cid_str).bold()
        ))
        .green()
    )
}

/// Print project IMPORT success message.
pub fn print_project_import_success() {
    println!(
        "{}",
        style(format!("project imported to local area")).green()
    )
}

/// Print project RUN success message
pub fn print_pipeline_run_success() {
    println!(
        "{}",
        style("successfully ran the transformation pipeline").green()
    )
}

/// Print project RUN export success message. Parameter is a vector of tuples containing the node
/// type name and the file written.
pub fn print_pipeline_export_success(node_exports: &[(String, String)]) {
    println!(
        "{}",
        style(format!(
            "{} successful export(s) during execution:",
            node_exports.len()
        ))
        .green()
    );
    for (node_typed_name, file_path) in node_exports.iter() {
        println!(
            "{}",
            style(format!("{} ??? {}", node_typed_name, file_path)).green()
        )
    }
}
