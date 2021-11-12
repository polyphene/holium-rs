//! Initialize a repository of Holium objects stored on the file system.

use std::io::Write;
use std::path::PathBuf;
use std::{env, fs};

use anyhow::Result;
use clap::{App, Arg, ArgMatches, SubCommand};
use console::style;
use thiserror::Error;

use crate::utils::repo::constants::{HOLIUM_DIR, INTERPLANETARY_DIR, LOCAL_DIR, PORTATIONS_FILE};

#[derive(Error, Debug)]
/// errors
enum CmdError {
    /// Thrown when trying to initialize a repository twice, without the force option.
    #[error("failed to initiate as '.holium' already exists. Use `-f` to force.")]
    AlreadyInitializedRepo,
    /// Thrown when trying to initialize a repository that is not tracked by any supported SCM tool, without the dedicated option.
    #[error("failed to initiate as current repository is not tracked by any SCM tool. Use `--no-scm` to initialize anyway.")]
    NotScmTracked,
    /// Thrown when trying to initialize a repository that is not tracked by any supported DVC tool, without the dedicated option.
    #[error("failed to initiate as current repository is not tracked by any DVC tool. Use `--no-dvc` to initialize anyway.")]
    NotDvcTracked,
}

/// command
pub(crate) fn cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("init")
        .about("Initializes a repository of Holium objects")
        .args(&[
            Arg::with_name("no-scm")
                .help("Initiate Holium in directory that is not tracked by any SCM tool")
                .long("no-scm"),
            Arg::with_name("no-dvc")
                .help("Initiate Holium in directory that is not tracked by any DVC tool")
                .long("no-dvc"),
            Arg::with_name("force")
                .help("Overwrites existing Holium project")
                .short("f")
                .long("force"),
        ])
}

/// handler
pub(crate) fn handle_cmd(matches: &ArgMatches) -> Result<()> {
    // Get path to current directory
    let cur_dir = env::current_dir()?;
    // Initialize a Holium repository in current directory
    let no_scm = matches.is_present("no-scm");
    let no_dvc = matches.is_present("no-dvc");
    let force = matches.is_present("force");
    init(&cur_dir, no_scm, no_dvc, force)
}

/// Creates a new empty repository on the given directory, basically creating a `.holium` directory.
///
/// It is recommended to track the repository with a SCM and a data version control tool. Otherwise,
/// the options `--no-scm` and/or `--no-dvc` should be used.
///
/// In case the directory is not empty, the `--force` option must be used in order to override it.
fn init(root_dir: &PathBuf, no_scm: bool, no_dvc: bool, force: bool) -> Result<()> {
    // If root directory is already an initialized repository, force re-initialization or throw an error
    let local_holium_path = root_dir.join(HOLIUM_DIR);
    if local_holium_path.exists() {
        if force {
            if local_holium_path.is_dir() {
                fs::remove_dir_all(local_holium_path)?;
            } else {
                fs::remove_file(local_holium_path)?;
            }
        } else {
            return Err(CmdError::AlreadyInitializedRepo.into());
        }
    }

    // Check if the repository is tracked with an SCM and/or a Data Version Control tool
    let is_scm_enabled = root_dir.join(".git").exists();
    let is_dvc_enabled = root_dir.join(".dvc").exists();

    // Enforce usage with an SCM and/or a Data Version Control tool, or with appropriate forcing options
    verify_scm_and_dvc_usage(is_scm_enabled, is_dvc_enabled, no_scm, no_dvc)?;

    // Create project structure
    create_project_structure(&root_dir, is_scm_enabled, is_dvc_enabled)?;

    Ok(())
}

fn create_project_structure(
    root_dir: &PathBuf,
    is_scm_enabled: bool,
    is_dvc_enabled: bool,
) -> Result<()> {
    // Create project structure
    let holium_dir = root_dir.join(HOLIUM_DIR);
    fs::create_dir(&holium_dir)?;
    fs::create_dir(&holium_dir.join(INTERPLANETARY_DIR))?;
    fs::create_dir(&holium_dir.join(LOCAL_DIR))?;
    fs::File::create(&holium_dir.join(PORTATIONS_FILE))?;

    // Add a .gitignore file
    if is_scm_enabled {
        let gitignore_file = fs::File::create(&holium_dir.join(".gitignore"))?;
        writeln!(&gitignore_file, "{}", LOCAL_DIR)?;
    }

    // Advise on running the tracking tool(s) once
    advise_to_track(is_scm_enabled, is_dvc_enabled);

    // Print success message
    println!("Initialized Holium repository.");

    Ok(())
}

/// Advise on running the appropriate tracking tool(s) once at initialisation
fn advise_to_track(is_scm_enabled: bool, is_dvc_enabled: bool) {
    if !is_scm_enabled && !is_dvc_enabled {
        return;
    }
    println!("To track changes in the Holium project, run :\n");
    if is_dvc_enabled {
        println!("\tdvc add {}/{}", HOLIUM_DIR, INTERPLANETARY_DIR);
    }
    if is_scm_enabled {
        println!("\tgit add {}", HOLIUM_DIR);
    }
    println!()
}

fn verify_scm_and_dvc_usage(
    is_scm_enabled: bool,
    is_dvc_enabled: bool,
    no_scm: bool,
    no_dvc: bool,
) -> Result<()> {
    if !is_scm_enabled && !no_scm {
        return Err(CmdError::NotScmTracked.into());
    }
    if !is_dvc_enabled && !no_dvc {
        return Err(CmdError::NotDvcTracked.into());
    }
    if is_scm_enabled && !is_dvc_enabled {
        // Warn against the use of SCM with no DVC tool
        println!("{}", style("Initializing a repository without data version control may lead to commit large files.\nConsider using DVC : https://dvc.org/\n").yellow())
    }
    Ok(())
}
