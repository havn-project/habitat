use clap_v4 as clap;

use clap::Parser;

use habitat_common::{cli::clap_validators::HabPkgIdentValueParser,
                     consts::PRODUCT_NAME};
use habitat_core::package::PackageIdent;

use crate::{cli_v4::utils::RemoteSup,
            error::Result as HabResult,
            gateway_util};

#[derive(Clone, Debug, Parser)]
#[command(author = habitat_common::consts::CLI_AUTHOR,
          about = format!("Start a loaded, but stopped, {} service", PRODUCT_NAME),
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct StartCommand {
    /// A package identifier (ex: core/redis, core/busybox-static/1.42.2)
    #[arg(name = "PKG_IDENT", value_parser = HabPkgIdentValueParser::simple())]
    pkg_ident: PackageIdent,

    #[command(flatten)]
    remote_sup: RemoteSup,
}

impl StartCommand {
    pub(crate) async fn do_command(&self) -> HabResult<()> {
        let remote_sup = self.remote_sup.clone();
        let msg =
            habitat_sup_protocol::ctl::SvcStart { ident: Some(self.pkg_ident.clone().into()), };
        gateway_util::send(remote_sup.inner(), msg).await
    }
}
