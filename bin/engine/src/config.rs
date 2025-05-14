// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use std::env::args;
use std::path::PathBuf;
use std::str::FromStr;

use common::repo::pool::PostgresConfig;
use common::ConfigValue;
use log::info;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub handle: Option<HandleConfig>,
    pub rule: Option<RuleConfig>,
    pub rule_pumpfun: Option<RulePumpfunConfig>,
    pub rule_pumpup: Option<RulePumpupConfig>,
    pub tokio: TokioConfig,
}

#[derive(Debug, Deserialize)]
pub struct TokioConfig {
    pub threads: ConfigValue,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HandleConfig {
    pub active: ConfigValue,
    pub mode: ConfigValue,
    pub secret: ConfigValue,

    pub rpc_url: ConfigValue,

    pub connection_string: ConfigValue,
    pub pool_min: ConfigValue,
    pub pool_max: ConfigValue,
    pub timeout_acquire_ms: ConfigValue,
}

impl From<HandleConfig> for PostgresConfig {
    fn from(value: HandleConfig) -> Self {
        Self {
            connection_string: value.connection_string,
            pool_min: value.pool_min,
            pool_max: value.pool_max,
            timeout_acquire_ms: value.timeout_acquire_ms,
        }
    }
}

impl Default for HandleConfig {
    fn default() -> Self {
        Self {
            active: ConfigValue::value(false),
            mode: ConfigValue::default(),
            secret: ConfigValue::default(),
            rpc_url: ConfigValue::default(),
            connection_string: ConfigValue::default(),
            pool_min: ConfigValue::default(),
            pool_max: ConfigValue::default(),
            timeout_acquire_ms: ConfigValue::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RulePumpfunConfig {
    pub active: ConfigValue,
}

impl Default for RulePumpfunConfig {
    fn default() -> Self {
        Self {
            active: ConfigValue::value(false),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RulePumpupConfig {
    pub active: ConfigValue,
}

impl Default for RulePumpupConfig {
    fn default() -> Self {
        Self {
            active: ConfigValue::value(false),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RuleConfig {
    pub connection_string: ConfigValue,
    pub pool_min: ConfigValue,
    pub pool_max: ConfigValue,
    pub timeout_acquire_ms: ConfigValue,
}

impl From<RuleConfig> for PostgresConfig {
    fn from(value: RuleConfig) -> Self {
        Self {
            connection_string: value.connection_string,
            pool_min: value.pool_min,
            pool_max: value.pool_max,
            timeout_acquire_ms: value.timeout_acquire_ms,
        }
    }
}

impl Default for RuleConfig {
    fn default() -> Self {
        Self {
            connection_string: ConfigValue::default(),
            pool_min: ConfigValue::default(),
            pool_max: ConfigValue::default(),
            timeout_acquire_ms: ConfigValue::default(),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let args: Vec<String> = args().collect();

        let config_path = if args.len() == 2 {
            PathBuf::from_str(args.get(1).unwrap()).unwrap()
        } else {
            let path = PathBuf::from_str(args.first().unwrap()).unwrap();
            path.parent().unwrap().join("config.toml")
        };

        info!("Loads {}", config_path.to_string_lossy());
        let config = std::fs::read_to_string(config_path).expect("Unable to read config");

        toml::from_str(&config).expect("Unable to parse config")
    }
}
