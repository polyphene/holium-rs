use anyhow::{Context, Result};
use std::io::Write;
use console::style;
use thiserror::Error;

#[derive(Debug, Error)]
enum Error {
    #[error("error while writing success message")]
    FailedToWriteSuccessMessage,
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
    ).context(Error::FailedToWriteSuccessMessage)
}

/// Print DELETE method success message.
pub fn print_delete_success(writter: &mut Write, key: &str) -> Result<()> {
    writeln!(writter,
             "{}",
             style(format!("object deleted: {}", style(key).bold())).green()
    ).context(Error::FailedToWriteSuccessMessage)
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
}