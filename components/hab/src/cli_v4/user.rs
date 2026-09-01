use crate::error::Result as HabResult;
use clap::Subcommand;
use clap_v4 as clap;
use habitat_common::{consts::PRODUCT_NAME,
                     ui::UI};

mod key;
use key::UserKeyCommand;

#[derive(Debug, Clone, Subcommand)]
#[command(rename_all = "kebab-case",
          arg_required_else_help = true,
          author = habitat_common::consts::CLI_AUTHOR,
          about = format!("Commands relating to {} users", PRODUCT_NAME),
          help_template = "{name} {version} {author-section} \
                           {about-section}\n{usage-heading}\n{usage}\n\n{all-args}\n")]
pub(crate) enum UserCommand {
    #[command(subcommand,
              about = format!("Commands relating to {} user keys", PRODUCT_NAME))]
    Key(UserKeyCommand),
}

impl UserCommand {
    pub(crate) async fn do_command(&self, ui: &mut UI) -> HabResult<()> {
        match self {
            UserCommand::Key(cmd) => cmd.do_key(ui).await,
        }
    }
}
