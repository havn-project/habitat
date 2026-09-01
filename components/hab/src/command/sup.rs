use crate::{VERSION,
            common::ui::UI,
            error::{Error,
                    Result},
            exec,
            hcore::{crypto::init,
                    env as henv,
                    fs::find_command,
                    os::process,
                    package::PackageIdent}};
use habitat_common::consts::{DEFAULT_HAB_SUP_PKG_IDENT,
                             DEFAULT_SUP_CMD};
use std::{ffi::OsString,
          path::PathBuf,
          str::FromStr};

pub const SUP_CMD_ENVVAR: &str = "HAB_SUP_BINARY";

pub async fn start(ui: &mut UI, args: &[OsString]) -> Result<()> {
    let command = match henv::var(SUP_CMD_ENVVAR) {
        Ok(command) => PathBuf::from(command),
        Err(_) => {
            init()?;
            let version: Vec<&str> = VERSION.split('/').collect();
            exec::command_from_min_pkg(ui,
                                       DEFAULT_SUP_CMD,
                                       &PackageIdent::from_str(&format!("{}/{}",
                                                                        DEFAULT_HAB_SUP_PKG_IDENT,
                                                                        version[0]))?).await?
        }
    };
    if let Some(cmd) = find_command(&command) {
        process::become_command(cmd, args)?;
        Ok(())
    } else {
        Err(Error::ExecCommandNotFound(command))
    }
}
