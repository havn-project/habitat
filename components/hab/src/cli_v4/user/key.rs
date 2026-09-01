use crate::error::Result as HabResult;
use clap::Subcommand;
use clap_v4 as clap;
use habitat_common::{consts::PRODUCT_NAME,
                     ui::UI};

mod generate;
use generate::UserKeyGenerateOptions;

#[derive(Debug, Clone, Subcommand)]
#[command(rename_all = "kebab-case",
          arg_required_else_help = true,
          author = habitat_common::consts::CLI_AUTHOR,
          about = format!("Commands relating to {} user keys", PRODUCT_NAME),
          help_template = "{name} {version} {author-section} \
                           {about-section}\n{usage-heading}\n{usage}\n\n{all-args}\n")]
pub(crate) enum UserKeyCommand {
    #[command(about = format!("Generates a {} user key", PRODUCT_NAME))]
    Generate(UserKeyGenerateOptions),
}

impl UserKeyCommand {
    pub(crate) async fn do_key(&self, ui: &mut UI) -> HabResult<()> {
        match self {
            UserKeyCommand::Generate(opts) => opts.do_generate(ui).await,
        }
    }
}
