use console::style;

/*
 Success messages
 */

/// Print CREATE method success message.
pub fn print_create_success(key: &str) {
    println!("{}", style(format!("new object created: {}", style(key).bold())).green())
}

/// Print UPDATE method success message.
pub fn print_update_success(key: &str) {
    println!("{}", style(format!("object updated: {}", style(key).bold())).green())
}

/// Print DELETE method success message.
pub fn print_delete_success(key: &str) {
    println!("{}", style(format!("object deleted: {}", style(key).bold())).green())
}