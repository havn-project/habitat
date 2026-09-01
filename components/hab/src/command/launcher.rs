use crate::{VERSION,
            command::sup::SUP_CMD_ENVVAR,
            common::ui::UI,
            error::{Error,
                    Result},
            exec,
            hcore::{crypto::init,
                    env as henv,
                    fs::find_command,
                    os::process,
                    package::PackageIdent}};
use habitat_common::consts::{DEFAULT_HAB_LAUNCHER_PKG_IDENT,
                             DEFAULT_HAB_SUP_PKG_IDENT,
                             DEFAULT_LAUNCHER_CMD,
                             DEFAULT_SUP_CMD};

use std::{ffi::OsString,
          path::PathBuf,
          str::FromStr};

use crate::cli_v4::sup::sup_run::SupRunOptions;

const LAUNCH_CMD_ENVVAR: &str = "HAB_LAUNCH_BINARY";

pub(crate) async fn start_v4(ui: &mut UI, sup_run: SupRunOptions, args: &[OsString]) -> Result<()> {
    init()?;
    let channel = sup_run.shared_load.channel.unwrap_or_default();
    if henv::var(SUP_CMD_ENVVAR).is_err() {
        let version: Vec<&str> = VERSION.split('/').collect();
        exec::command_from_min_pkg_with_channel(ui,
                                                DEFAULT_SUP_CMD,
                                                &PackageIdent::from_str(&format!("{}/{}",
                                                                                 DEFAULT_HAB_SUP_PKG_IDENT,
                                                                                 version[0]))?,
                                                channel.clone()).await?;
    }
    let command = match henv::var(LAUNCH_CMD_ENVVAR) {
        Ok(command) => PathBuf::from(command),
        Err(_) => {
            init()?;
            exec::command_from_min_pkg_with_channel(ui,
                                                    DEFAULT_LAUNCHER_CMD,
                                                    &PackageIdent::from_str(DEFAULT_HAB_LAUNCHER_PKG_IDENT)?,
                                                    channel).await?
        }
    };
    if let Some(cmd) = find_command(&command) {
        process::become_command(cmd, args)?;
        Ok(())
    } else {
        Err(Error::ExecCommandNotFound(command))
    }
}
