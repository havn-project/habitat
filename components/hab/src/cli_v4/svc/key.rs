use clap_v4 as clap;

use std::path::PathBuf;

use clap::Parser;

use habitat_common::{consts::{CLI_AUTHOR,
                              PRODUCT_NAME},
                     ui::UI};
use habitat_core::{crypto::keys::KeyCache,
                   service::ServiceGroup};

use crate::{cli_v4::utils::CacheKeyPath,
            command::service::key::generate::start,
            error::Result as HabResult};

#[derive(Clone, Debug, Parser)]
#[command(author = CLI_AUTHOR,
          about = format!("Commands relating to {} service keys", PRODUCT_NAME),
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) enum KeyCommand {
    Generate(KeyGenerate),
}

#[derive(Clone, Debug, Parser)]
#[command(author = CLI_AUTHOR,
          about = format!("Generates a {} service key", PRODUCT_NAME),
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct KeyGenerate {
    /// Target service group service.group[@organization] (ex: redis.default or
    /// foo.default@bazcorp)
    #[arg(name = "SERVICE_GROUP")]
    service_group: ServiceGroup,

    /// The service organization
    #[structopt(name = "ORG", env = "HABITAT_ORG")]
    org: String,

    #[command(flatten)]
    cache_key_path: CacheKeyPath,
}

impl KeyGenerate {
    pub(crate) async fn do_command(&self, ui: &mut UI) -> HabResult<()> {
        let key_cache = KeyCache::new::<PathBuf>((&self.cache_key_path).into());
        key_cache.setup()?;

        start(ui, &self.org, &self.service_group, &key_cache)
    }
}
