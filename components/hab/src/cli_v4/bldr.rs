use crate::error::Result as HabResult;
use clap::Subcommand;
use clap_v4 as clap;
use habitat_common::{consts::PRODUCT_NAME,
                     ui::UI};

mod channel;
use channel::ChannelCommand;

#[derive(Debug, Clone, Subcommand)]
#[command(rename_all = "kebab-case",
          arg_required_else_help = true,
          author = habitat_common::consts::CLI_AUTHOR,
          about = format!("Commands relating to {} Builder", PRODUCT_NAME),
          help_template = "{name} {version} {author-section} \
                           {about-section}\n{usage-heading}\n{usage}\n\n{all-args}\n")]
pub(crate) enum BldrCommand {
    #[command(subcommand,
              about = format!("Commands relating to {} Builder channels", PRODUCT_NAME))]
    Channel(ChannelCommand),
}

impl BldrCommand {
    pub async fn do_command(&self, ui: &mut UI) -> HabResult<()> {
        match self {
            BldrCommand::Channel(cmd) => cmd.do_command(ui).await,
        }
    }
}
