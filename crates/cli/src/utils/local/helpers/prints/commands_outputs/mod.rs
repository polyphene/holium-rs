use crate::utils::interplanetary::multiformats::DEFAULT_MULTIBASE;
use anyhow::{Context, Result};
use cid::Cid;
use console::style;
use std::io::Write;
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
    #[error("error while writing project export success message")]
    FailedToWriteProjectExportSuccessMessage,
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
    )
    .context(Error::FailedToWriteSuccessMessage)
}

/// Print UPDATE method success message.
pub fn print_update_success(writter: &mut Write, key: &str) -> Result<()> {
    writeln!(
        writter,
        "{}",
        style(format!("object updated: {}", style(key).bold())).green()
    )
    .context(Error::FailedToWriteUpdateMessage)
}

/// Print DELETE method success message.
pub fn print_delete_success(writter: &mut Write, key: &str) -> Result<()> {
    writeln!(
        writter,
        "{}",
        style(format!("object deleted: {}", style(key).bold())).green()
    )
    .context(Error::FailedToWriteDeleteMessage)
}

/// Print success message for methods checking the health of the transformation pipeline currently
/// in the local area.
pub fn print_pipeline_health_success(writter: &mut Write) -> Result<()> {
    writeln!(
        writter,
        "{}",
        style("current project holds a healthy transformation pipeline").green()
    )
    .context(Error::FailedToWriteHealthSuccessMessage)
}

/// Print project EXPORT success message.
pub fn print_project_export_success(writter: &mut Write, cid: &Cid) -> Result<()> {
    let cid_str = cid
        .to_string_of_base(DEFAULT_MULTIBASE)
        .unwrap_or("".to_string());
    writeln!(
        writter,
        "{}",
        style(format!(
            "project exported with pipeline cid: {}",
            style(cid_str).bold()
        ))
        .green()
    )
    .context(Error::FailedToWriteProjectExportSuccessMessage)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

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

    #[test]
    fn can_have_formatted_msg_for_project_export_success() {
        let mut stdout = Vec::new();
        let cid_str = "bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi";
        let cid =
            Cid::from_str("bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi").unwrap();
        let awaited_msg = format!("project exported with pipeline cid: {}\n", cid_str);

        // pass fake stdout when calling when testing
        print_project_export_success(&mut stdout, &cid).unwrap();
        println!("{:?}", String::from_utf8(stdout.clone()));
        assert_eq!(awaited_msg.as_bytes(), stdout);
    }
}
