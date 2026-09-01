// To handle basic commands such as launching a shell or terminating

use clap_v4 as clap;

use clap::Parser;

use crate::error::Result as HabResult;
use habitat_common::{consts::PRODUCT_NAME,
                     ui::UI};

use std::ffi::OsString;

#[cfg(not(target_os = "macos"))]
use crate::command;

#[derive(Debug, Clone, Parser)]
#[command(author = habitat_common::consts::CLI_AUTHOR,
          about = format!("Gracefully terminate the {} Supervisor and all of its running services", PRODUCT_NAME),
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct SupTermCommand {
    #[arg()]
    args: Vec<OsString>,
}

impl SupTermCommand {
    #[cfg(not(target_os = "macos"))]
    pub(super) async fn execute(&self, ui: &mut UI) -> HabResult<()> {
        let mut args = vec!["term".into()];
        args.extend(self.args.clone());
        return command::sup::start(ui, &args).await;
    }

    #[cfg(target_os = "macos")]
    pub(super) async fn execute(&self, _ui: &mut UI) -> HabResult<()> { Ok(()) }
}
