use anyhow::{Context, Result};
use std::io::Write;
use console::style;
use thiserror::Error;

#[derive(Debug, Error)]
enum Error {
    #[error("error while writing success message")]
    FailedToWriteSuccessMessage,
    #[error("error while writing update message")]
    FailedToWriteUpdateMessage,
    #[error("error while writing delete message")]
    FailedToWriteDeleteMessage,
    #[error("error while writing health success message")]
    FailedToWriteHealthSuccessMessage,
}

/*
 Success messages
 */

/// Print CREATE method success message.
pub fn print_create_success(writter: &mut Write, key: &str) -> Result<()> {
    writeln!(
        writter,
        "{}",
        style(format!("new object created: {}", style(key).bold())).green()
    ).context(Error::FailedToWriteSuccessMessage)
}

/// Print UPDATE method success message.
pub fn print_update_success(writter: &mut Write, key: &str) -> Result<()> {
    writeln!(writter,
             "{}",
             style(format!("object updated: {}", style(key).bold())).green()
    ).context(Error::FailedToWriteUpdateMessage)
}

/// Print DELETE method success message.
pub fn print_delete_success(writter: &mut Write, key: &str) -> Result<()> {
    writeln!(writter,
             "{}",
             style(format!("object deleted: {}", style(key).bold())).green()
    ).context(Error::FailedToWriteDeleteMessage)
}

/// Print success message for methods checking the health of the transformation pipeline currently
/// in the local area.
pub fn print_pipeline_health_success(writter: &mut Write) -> Result<()> {
    writeln!(
        writter,
        "{}",
        style("current project holds a healthy transformation pipeline").green()
    ).context(Error::FailedToWriteHealthSuccessMessage)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_have_formatted_msg_for_create_success() {
        let mut stdout = Vec::new();
        let key = "my_key";
        let awaited_msg = format!("new object created: {}\n", &key);

        // pass fake stdout when calling when testing
        print_create_success(&mut stdout, key).unwrap();

        assert_eq!(awaited_msg.as_bytes(), stdout);
    }

    #[test]
    fn can_have_formatted_msg_for_update_success() {
        let mut stdout = Vec::new();
        let key = "my_key";
        let awaited_msg = format!("object updated: {}\n", &key);

        // pass fake stdout when calling when testing
        print_update_success(&mut stdout, key).unwrap();

        assert_eq!(awaited_msg.as_bytes(), stdout);
    }

    #[test]
    fn can_have_formatted_msg_for_delete_success() {
        let mut stdout = Vec::new();
        let key = "my_key";
        let awaited_msg = format!("object deleted: {}\n", &key);

        // pass fake stdout when calling when testing
        print_delete_success(&mut stdout, key).unwrap();

        assert_eq!(awaited_msg.as_bytes(), stdout);
    }

    #[test]
    fn can_have_formatted_msg_for_health_success() {
        let mut stdout = Vec::new();
        let awaited_msg = "current project holds a healthy transformation pipeline\n";

        // pass fake stdout when calling when testing
        print_pipeline_health_success(&mut stdout).unwrap();

        assert_eq!(awaited_msg.as_bytes(), stdout);
    }
}

