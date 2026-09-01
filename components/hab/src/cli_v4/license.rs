use clap_v4 as clap;

use crate::{error::Result as HabResult,
            license};
use clap::Subcommand;
use habitat_common::{consts::PRODUCT_NAME,
                     ui::UI};

#[derive(Clone, Debug, Subcommand)]
#[command(author = habitat_common::consts::CLI_AUTHOR,
          about = format!("Commands relating to {} license agreements", PRODUCT_NAME),
          arg_required_else_help = true,
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(super) enum LicenseCommand {
    /// Accept the Chef Binary Distribution Agreement without prompting
    Accept,
}

impl LicenseCommand {
    pub(crate) async fn do_command(&self, ui: &mut UI) -> HabResult<()> {
        match self {
            Self::Accept => {
                license::accept_license(ui)?;
                Ok(())
            }
        }
    }
}
