use dns_common::tracing::TracingConfig;

use crate::config;
use crate::prelude::*;
use dns_common::deser::yaml_from_path as from_path;

static YAML_FILE_NAME: &str = "config.yaml";

#[derive(Deserialize, Serialize, Default)]
#[serde(deny_unknown_fields)]
pub struct YamlConfig {
    pub tracing: Option<TracingConfig>,
}

impl YamlConfig {
    pub fn new(env: &EnvConfig, clap: &ClapConfig) -> Result<Self> {
        if let Some(p) = clap.config.clone() {
            if p.exists() {
                match from_path(&p) {
                    Ok(c) => return Ok(c),
                    Err(e) => eprintln!("invalid config path: {}, {}", p.to_string(), e),
                }
            }
        }

        if let Some(path_str) = env.config_path.as_ref() {
            let p = PathBuf::from(path_str);
            if p.exists() {
                match from_path(&p) {
                    Ok(c) => return Ok(c),
                    Err(e) => eprintln!("invalid config path: {}, {}", p.to_string(), e),
                }
            }
        }

        let config_path = default_config_path();
        if let Ok(p) = config_path {
            if p.exists() {
                match from_path(&p) {
                    Ok(c) => return Ok(c),
                    Err(e) => eprintln!("invalid config path: {}, {}", p.to_string(), e),
                }
            }
        }

        match clap.cmd {
            config::clap::Cmd::Config(_) => Ok(YamlConfig::default()),
            _ => Ok(YamlConfig::default()),
            /*
                        _ => Err(anyhow!(
                            "No valid config.

            Please run the following to create a file with default values:

               {} config init",
                            PROJECT_NAME
                        )),
                        */
        }
    }
}

pub fn default_config_path() -> Result<PathBuf> {
    super::default_dir().map(|p| p.join(YAML_FILE_NAME))
}
