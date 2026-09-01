//! Common constants like `hab` package name etc.

/// Default package origin
pub const DEFAULT_ORIGIN: &str = "chef";

/// Default `hab` package ident
pub const DEFAULT_HAB_PKG_IDENT: &str = "chef/hab";

/// Default `hab-sup` package ident
pub const DEFAULT_HAB_SUP_PKG_IDENT: &str = "chef/hab-sup";

/// Default `hab-launcher` package ident
pub const DEFAULT_HAB_LAUNCHER_PKG_IDENT: &str = "chef/hab-launcher";

/// Default `hab-studio` package ident
pub const DEFAULT_HAB_STUDIO_PKG_IDENT: &str = "chef/hab-studio";

/// Default Supervisor binary name, searched for inside `DEFAULT_HAB_SUP_PKG_IDENT`
pub const DEFAULT_SUP_CMD: &str = "hab-sup";

/// Default Launcher binary name, searched for inside `DEFAULT_HAB_LAUNCHER_PKG_IDENT`
pub const DEFAULT_LAUNCHER_CMD: &str = "hab-launch";

/// Default Studio binary name, searched for inside `DEFAULT_HAB_STUDIO_PKG_IDENT`
pub const DEFAULT_STUDIO_CMD: &str = "hab-studio";

/// Default path to a Supervisor's config file, if one exists
pub const DEFAULT_SUP_CONFIG_PATH: &str = "/hab/sup/default/config/sup.toml";

/// Default Docker Hub image for a Linux Studio container
pub const DEFAULT_DOCKER_STUDIO_IMAGE: &str = "habitat/default-studio";

/// Default Docker Hub image for a Windows Studio container
pub const DEFAULT_DOCKER_STUDIO_WINDOWS_IMAGE: &str = "habitat/win-studio";

/// Default builder URL
pub const DEFAULT_BUILDER_URL: &str = "https://bldr.habitat.sh";

/// Default documentation base URL
pub const DEFAULT_DOCS_URL: &str = "https://www.habitat.sh/docs";

/// CLI binary name
pub const CLI_NAME: &str = "hab";

/// Product name
pub const PRODUCT_NAME: &str = "Habitat";

/// CLI author/maintainer line
pub const CLI_AUTHOR: &str = "\nThe Habitat Maintainers <humans@habitat.sh>";

/// CLI top-level about text
pub const CLI_ABOUT: &str = "Patents: https://chef.io/patents\n\"A Habitat is the natural \
                             environment for your services\" - Alan Turing";
