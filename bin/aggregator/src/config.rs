// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use std::env::args;
use std::path::PathBuf;
use std::str::FromStr;

use common::repo::pool::PostgresConfig;
use common::ConfigValue;
use log::info;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub tokio: TokioConfig,
    pub leaderboard: Option<LeaderboardConfig>,

    pub jupiter_candle: Option<JupiterCandleConfig>,
    pub jupiter_mcap: Option<JupiterMcapConfig>,
    pub jupiter_twap: Option<JupiterTwapConfig>,
    pub jupiter_usd: Option<JupiterUsdConfig>,

    pub pumpfun_candle: Option<PumpfunCandleConfig>,
    pub pumpfun_mcap: Option<PumpfunMcapConfig>,
    pub pumpfun_progress: Option<PumpfunProgressConfig>,
    pub pumpfun_summary: Option<PumpfunSummaryConfig>,
    pub pumpfun_twap: Option<PumpfunTwapConfig>,
    pub pumpfun_usd: Option<PumpfunUsdConfig>,

    pub pumpswap_candle: Option<PumpswapCandleConfig>,
    pub pumpswap_mcap: Option<PumpswapMcapConfig>,
    pub pumpswap_summary: Option<PumpswapSummaryConfig>,
    pub pumpswap_twap: Option<PumpswapTwapConfig>,
    pub pumpswap_usd: Option<PumpswapUsdConfig>,

    pub pumpup_candle: Option<PumpupCandleConfig>,
    pub pumpup_mcap: Option<PumpupMcapConfig>,
    pub pumpup_progress: Option<PumpupProgressConfig>,
    pub pumpup_summary: Option<PumpupSummaryConfig>,
    pub pumpup_twap: Option<PumpupTwapConfig>,
    pub pumpup_usd: Option<PumpupUsdConfig>,

    pub solana_sol: Option<SolanaSolConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LeaderboardConfig {
    pub active: ConfigValue,
    pub mode: ConfigValue,

    pub connection_string: ConfigValue,
    pub pool_min: ConfigValue,
    pub pool_max: ConfigValue,
    pub timeout_acquire_ms: ConfigValue,
}

impl From<LeaderboardConfig> for PostgresConfig {
    fn from(value: LeaderboardConfig) -> Self {
        Self {
            connection_string: value.connection_string,
            pool_min: value.pool_min,
            pool_max: value.pool_max,
            timeout_acquire_ms: value.timeout_acquire_ms,
        }
    }
}

impl Default for LeaderboardConfig {
    fn default() -> Self {
        Self {
            active: ConfigValue::value(false),
            mode: ConfigValue::default(),
            connection_string: ConfigValue::default(),
            pool_min: ConfigValue::default(),
            pool_max: ConfigValue::default(),
            timeout_acquire_ms: ConfigValue::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TokioConfig {
    pub threads: ConfigValue,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JupiterCandleConfig {
    pub active: ConfigValue,
    pub mode: ConfigValue,

    pub connection_string: ConfigValue,
    pub pool_min: ConfigValue,
    pub pool_max: ConfigValue,
    pub timeout_acquire_ms: ConfigValue,
}

impl From<JupiterCandleConfig> for PostgresConfig {
    fn from(value: JupiterCandleConfig) -> Self {
        Self {
            connection_string: value.connection_string,
            pool_min: value.pool_min,
            pool_max: value.pool_max,
            timeout_acquire_ms: value.timeout_acquire_ms,
        }
    }
}

impl Default for JupiterCandleConfig {
    fn default() -> Self {
        Self {
            active: ConfigValue::value(false),
            mode: ConfigValue::default(),
            connection_string: ConfigValue::default(),
            pool_min: ConfigValue::default(),
            pool_max: ConfigValue::default(),
            timeout_acquire_ms: ConfigValue::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct JupiterMcapConfig {
    pub active: ConfigValue,
    pub mode: ConfigValue,

    pub connection_string: ConfigValue,
    pub pool_min: ConfigValue,
    pub pool_max: ConfigValue,
    pub timeout_acquire_ms: ConfigValue,
}

impl From<JupiterMcapConfig> for PostgresConfig {
    fn from(value: JupiterMcapConfig) -> Self {
        Self {
            connection_string: value.connection_string,
            pool_min: value.pool_min,
            pool_max: value.pool_max,
            timeout_acquire_ms: value.timeout_acquire_ms,
        }
    }
}

impl Default for JupiterMcapConfig {
    fn default() -> Self {
        Self {
            active: ConfigValue::value(false),
            mode: ConfigValue::default(),
            connection_string: ConfigValue::default(),
            pool_min: ConfigValue::default(),
            pool_max: ConfigValue::default(),
            timeout_acquire_ms: ConfigValue::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct JupiterTwapConfig {
    pub active: ConfigValue,
    pub mode: ConfigValue,

    pub connection_string: ConfigValue,
    pub pool_min: ConfigValue,
    pub pool_max: ConfigValue,
    pub timeout_acquire_ms: ConfigValue,
}

impl From<JupiterTwapConfig> for PostgresConfig {
    fn from(value: JupiterTwapConfig) -> Self {
        Self {
            connection_string: value.connection_string,
            pool_min: value.pool_min,
            pool_max: value.pool_max,
            timeout_acquire_ms: value.timeout_acquire_ms,
        }
    }
}

impl Default for JupiterTwapConfig {
    fn default() -> Self {
        Self {
            active: ConfigValue::value(false),
            mode: ConfigValue::default(),
            connection_string: ConfigValue::default(),
            pool_min: ConfigValue::default(),
            pool_max: ConfigValue::default(),
            timeout_acquire_ms: ConfigValue::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct JupiterUsdConfig {
    pub active: ConfigValue,
    pub mode: ConfigValue,

    pub connection_string: ConfigValue,
    pub pool_min: ConfigValue,
    pub pool_max: ConfigValue,
    pub timeout_acquire_ms: ConfigValue,
}

impl From<JupiterUsdConfig> for PostgresConfig {
    fn from(value: JupiterUsdConfig) -> Self {
        Self {
            connection_string: value.connection_string,
            pool_min: value.pool_min,
            pool_max: value.pool_max,
            timeout_acquire_ms: value.timeout_acquire_ms,
        }
    }
}

impl Default for JupiterUsdConfig {
    fn default() -> Self {
        Self {
            active: ConfigValue::value(false),
            mode: ConfigValue::default(),
            connection_string: ConfigValue::default(),
            pool_min: ConfigValue::default(),
            pool_max: ConfigValue::default(),
            timeout_acquire_ms: ConfigValue::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PumpfunCandleConfig {
    pub active: ConfigValue,
    pub mode: ConfigValue,

    pub connection_string: ConfigValue,
    pub pool_min: ConfigValue,
    pub pool_max: ConfigValue,
    pub timeout_acquire_ms: ConfigValue,
}

impl From<PumpfunCandleConfig> for PostgresConfig {
    fn from(value: PumpfunCandleConfig) -> Self {
        Self {
            connection_string: value.connection_string,
            pool_min: value.pool_min,
            pool_max: value.pool_max,
            timeout_acquire_ms: value.timeout_acquire_ms,
        }
    }
}

impl Default for PumpfunCandleConfig {
    fn default() -> Self {
        Self {
            active: ConfigValue::value(false),
            mode: ConfigValue::default(),
            connection_string: ConfigValue::default(),
            pool_min: ConfigValue::default(),
            pool_max: ConfigValue::default(),
            timeout_acquire_ms: ConfigValue::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PumpfunMcapConfig {
    pub active: ConfigValue,
    pub mode: ConfigValue,

    pub connection_string: ConfigValue,
    pub pool_min: ConfigValue,
    pub pool_max: ConfigValue,
    pub timeout_acquire_ms: ConfigValue,
}

impl From<PumpfunMcapConfig> for PostgresConfig {
    fn from(value: PumpfunMcapConfig) -> Self {
        Self {
            connection_string: value.connection_string,
            pool_min: value.pool_min,
            pool_max: value.pool_max,
            timeout_acquire_ms: value.timeout_acquire_ms,
        }
    }
}

impl Default for PumpfunMcapConfig {
    fn default() -> Self {
        Self {
            active: ConfigValue::value(false),
            mode: ConfigValue::default(),
            connection_string: ConfigValue::default(),
            pool_min: ConfigValue::default(),
            pool_max: ConfigValue::default(),
            timeout_acquire_ms: ConfigValue::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PumpfunProgressConfig {
    pub active: ConfigValue,
    pub mode: ConfigValue,

    pub connection_string: ConfigValue,
    pub pool_min: ConfigValue,
    pub pool_max: ConfigValue,
    pub timeout_acquire_ms: ConfigValue,
}

impl From<PumpfunProgressConfig> for PostgresConfig {
    fn from(value: PumpfunProgressConfig) -> Self {
        Self {
            connection_string: value.connection_string,
            pool_min: value.pool_min,
            pool_max: value.pool_max,
            timeout_acquire_ms: value.timeout_acquire_ms,
        }
    }
}

impl Default for PumpfunProgressConfig {
    fn default() -> Self {
        Self {
            active: ConfigValue::value(false),
            mode: ConfigValue::default(),
            connection_string: ConfigValue::default(),
            pool_min: ConfigValue::default(),
            pool_max: ConfigValue::default(),
            timeout_acquire_ms: ConfigValue::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PumpfunSummaryConfig {
    pub active: ConfigValue,
    pub mode: ConfigValue,

    pub connection_string: ConfigValue,
    pub pool_min: ConfigValue,
    pub pool_max: ConfigValue,
    pub timeout_acquire_ms: ConfigValue,
}

impl From<PumpfunSummaryConfig> for PostgresConfig {
    fn from(value: PumpfunSummaryConfig) -> Self {
        Self {
            connection_string: value.connection_string,
            pool_min: value.pool_min,
            pool_max: value.pool_max,
            timeout_acquire_ms: value.timeout_acquire_ms,
        }
    }
}

impl Default for PumpfunSummaryConfig {
    fn default() -> Self {
        Self {
            active: ConfigValue::value(false),
            mode: ConfigValue::default(),
            connection_string: ConfigValue::default(),
            pool_min: ConfigValue::default(),
            pool_max: ConfigValue::default(),
            timeout_acquire_ms: ConfigValue::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PumpfunTwapConfig {
    pub active: ConfigValue,
    pub mode: ConfigValue,

    pub connection_string: ConfigValue,
    pub pool_min: ConfigValue,
    pub pool_max: ConfigValue,
    pub timeout_acquire_ms: ConfigValue,
}

impl From<PumpfunTwapConfig> for PostgresConfig {
    fn from(value: PumpfunTwapConfig) -> Self {
        Self {
            connection_string: value.connection_string,
            pool_min: value.pool_min,
            pool_max: value.pool_max,
            timeout_acquire_ms: value.timeout_acquire_ms,
        }
    }
}

impl Default for PumpfunTwapConfig {
    fn default() -> Self {
        Self {
            active: ConfigValue::value(false),
            mode: ConfigValue::default(),
            connection_string: ConfigValue::default(),
            pool_min: ConfigValue::default(),
            pool_max: ConfigValue::default(),
            timeout_acquire_ms: ConfigValue::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PumpfunUsdConfig {
    pub active: ConfigValue,
    pub mode: ConfigValue,

    pub connection_string: ConfigValue,
    pub pool_min: ConfigValue,
    pub pool_max: ConfigValue,
    pub timeout_acquire_ms: ConfigValue,
}

impl From<PumpfunUsdConfig> for PostgresConfig {
    fn from(value: PumpfunUsdConfig) -> Self {
        Self {
            connection_string: value.connection_string,
            pool_min: value.pool_min,
            pool_max: value.pool_max,
            timeout_acquire_ms: value.timeout_acquire_ms,
        }
    }
}

impl Default for PumpfunUsdConfig {
    fn default() -> Self {
        Self {
            active: ConfigValue::value(false),
            mode: ConfigValue::default(),
            connection_string: ConfigValue::default(),
            pool_min: ConfigValue::default(),
            pool_max: ConfigValue::default(),
            timeout_acquire_ms: ConfigValue::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PumpswapCandleConfig {
    pub active: ConfigValue,
    pub mode: ConfigValue,

    pub connection_string: ConfigValue,
    pub pool_min: ConfigValue,
    pub pool_max: ConfigValue,
    pub timeout_acquire_ms: ConfigValue,
}

impl From<PumpswapCandleConfig> for PostgresConfig {
    fn from(value: PumpswapCandleConfig) -> Self {
        Self {
            connection_string: value.connection_string,
            pool_min: value.pool_min,
            pool_max: value.pool_max,
            timeout_acquire_ms: value.timeout_acquire_ms,
        }
    }
}

impl Default for PumpswapCandleConfig {
    fn default() -> Self {
        Self {
            active: ConfigValue::value(false),
            mode: ConfigValue::default(),
            connection_string: ConfigValue::default(),
            pool_min: ConfigValue::default(),
            pool_max: ConfigValue::default(),
            timeout_acquire_ms: ConfigValue::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PumpswapMcapConfig {
    pub active: ConfigValue,
    pub mode: ConfigValue,

    pub connection_string: ConfigValue,
    pub pool_min: ConfigValue,
    pub pool_max: ConfigValue,
    pub timeout_acquire_ms: ConfigValue,
}

impl From<PumpswapMcapConfig> for PostgresConfig {
    fn from(value: PumpswapMcapConfig) -> Self {
        Self {
            connection_string: value.connection_string,
            pool_min: value.pool_min,
            pool_max: value.pool_max,
            timeout_acquire_ms: value.timeout_acquire_ms,
        }
    }
}

impl Default for PumpswapMcapConfig {
    fn default() -> Self {
        Self {
            active: ConfigValue::value(false),
            mode: ConfigValue::default(),
            connection_string: ConfigValue::default(),
            pool_min: ConfigValue::default(),
            pool_max: ConfigValue::default(),
            timeout_acquire_ms: ConfigValue::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PumpswapSummaryConfig {
    pub active: ConfigValue,
    pub mode: ConfigValue,

    pub connection_string: ConfigValue,
    pub pool_min: ConfigValue,
    pub pool_max: ConfigValue,
    pub timeout_acquire_ms: ConfigValue,
}

impl From<PumpswapSummaryConfig> for PostgresConfig {
    fn from(value: PumpswapSummaryConfig) -> Self {
        Self {
            connection_string: value.connection_string,
            pool_min: value.pool_min,
            pool_max: value.pool_max,
            timeout_acquire_ms: value.timeout_acquire_ms,
        }
    }
}

impl Default for PumpswapSummaryConfig {
    fn default() -> Self {
        Self {
            active: ConfigValue::value(false),
            mode: ConfigValue::default(),
            connection_string: ConfigValue::default(),
            pool_min: ConfigValue::default(),
            pool_max: ConfigValue::default(),
            timeout_acquire_ms: ConfigValue::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PumpswapTwapConfig {
    pub active: ConfigValue,
    pub mode: ConfigValue,

    pub connection_string: ConfigValue,
    pub pool_min: ConfigValue,
    pub pool_max: ConfigValue,
    pub timeout_acquire_ms: ConfigValue,
}

impl From<PumpswapTwapConfig> for PostgresConfig {
    fn from(value: PumpswapTwapConfig) -> Self {
        Self {
            connection_string: value.connection_string,
            pool_min: value.pool_min,
            pool_max: value.pool_max,
            timeout_acquire_ms: value.timeout_acquire_ms,
        }
    }
}

impl Default for PumpswapTwapConfig {
    fn default() -> Self {
        Self {
            active: ConfigValue::value(false),
            mode: ConfigValue::default(),
            connection_string: ConfigValue::default(),
            pool_min: ConfigValue::default(),
            pool_max: ConfigValue::default(),
            timeout_acquire_ms: ConfigValue::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PumpswapUsdConfig {
    pub active: ConfigValue,
    pub mode: ConfigValue,

    pub connection_string: ConfigValue,
    pub pool_min: ConfigValue,
    pub pool_max: ConfigValue,
    pub timeout_acquire_ms: ConfigValue,
}

impl From<PumpswapUsdConfig> for PostgresConfig {
    fn from(value: PumpswapUsdConfig) -> Self {
        Self {
            connection_string: value.connection_string,
            pool_min: value.pool_min,
            pool_max: value.pool_max,
            timeout_acquire_ms: value.timeout_acquire_ms,
        }
    }
}

impl Default for PumpswapUsdConfig {
    fn default() -> Self {
        Self {
            active: ConfigValue::value(false),
            mode: ConfigValue::default(),
            connection_string: ConfigValue::default(),
            pool_min: ConfigValue::default(),
            pool_max: ConfigValue::default(),
            timeout_acquire_ms: ConfigValue::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SolanaSolConfig {
    pub active: ConfigValue,
    pub mode: ConfigValue,

    pub connection_string: ConfigValue,
    pub pool_min: ConfigValue,
    pub pool_max: ConfigValue,
    pub timeout_acquire_ms: ConfigValue,
}

impl From<SolanaSolConfig> for PostgresConfig {
    fn from(value: SolanaSolConfig) -> Self {
        Self {
            connection_string: value.connection_string,
            pool_min: value.pool_min,
            pool_max: value.pool_max,
            timeout_acquire_ms: value.timeout_acquire_ms,
        }
    }
}

impl Default for SolanaSolConfig {
    fn default() -> Self {
        Self {
            active: ConfigValue::value(false),
            mode: ConfigValue::default(),
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

#[derive(Debug, Clone, Deserialize)]
pub struct PumpupCandleConfig {
    pub active: ConfigValue,
    pub mode: ConfigValue,

    pub connection_string: ConfigValue,
    pub pool_min: ConfigValue,
    pub pool_max: ConfigValue,
    pub timeout_acquire_ms: ConfigValue,
}

impl From<PumpupCandleConfig> for PostgresConfig {
    fn from(value: PumpupCandleConfig) -> Self {
        Self {
            connection_string: value.connection_string,
            pool_min: value.pool_min,
            pool_max: value.pool_max,
            timeout_acquire_ms: value.timeout_acquire_ms,
        }
    }
}

impl Default for PumpupCandleConfig {
    fn default() -> Self {
        Self {
            active: ConfigValue::value(false),
            mode: ConfigValue::default(),
            connection_string: ConfigValue::default(),
            pool_min: ConfigValue::default(),
            pool_max: ConfigValue::default(),
            timeout_acquire_ms: ConfigValue::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PumpupMcapConfig {
    pub active: ConfigValue,
    pub mode: ConfigValue,

    pub connection_string: ConfigValue,
    pub pool_min: ConfigValue,
    pub pool_max: ConfigValue,
    pub timeout_acquire_ms: ConfigValue,
}

impl From<PumpupMcapConfig> for PostgresConfig {
    fn from(value: PumpupMcapConfig) -> Self {
        Self {
            connection_string: value.connection_string,
            pool_min: value.pool_min,
            pool_max: value.pool_max,
            timeout_acquire_ms: value.timeout_acquire_ms,
        }
    }
}

impl Default for PumpupMcapConfig {
    fn default() -> Self {
        Self {
            active: ConfigValue::value(false),
            mode: ConfigValue::default(),
            connection_string: ConfigValue::default(),
            pool_min: ConfigValue::default(),
            pool_max: ConfigValue::default(),
            timeout_acquire_ms: ConfigValue::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PumpupProgressConfig {
    pub active: ConfigValue,
    pub mode: ConfigValue,

    pub connection_string: ConfigValue,
    pub pool_min: ConfigValue,
    pub pool_max: ConfigValue,
    pub timeout_acquire_ms: ConfigValue,
}

impl From<PumpupProgressConfig> for PostgresConfig {
    fn from(value: PumpupProgressConfig) -> Self {
        Self {
            connection_string: value.connection_string,
            pool_min: value.pool_min,
            pool_max: value.pool_max,
            timeout_acquire_ms: value.timeout_acquire_ms,
        }
    }
}

impl Default for PumpupProgressConfig {
    fn default() -> Self {
        Self {
            active: ConfigValue::value(false),
            mode: ConfigValue::default(),
            connection_string: ConfigValue::default(),
            pool_min: ConfigValue::default(),
            pool_max: ConfigValue::default(),
            timeout_acquire_ms: ConfigValue::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PumpupSummaryConfig {
    pub active: ConfigValue,
    pub mode: ConfigValue,

    pub connection_string: ConfigValue,
    pub pool_min: ConfigValue,
    pub pool_max: ConfigValue,
    pub timeout_acquire_ms: ConfigValue,
}

impl From<PumpupSummaryConfig> for PostgresConfig {
    fn from(value: PumpupSummaryConfig) -> Self {
        Self {
            connection_string: value.connection_string,
            pool_min: value.pool_min,
            pool_max: value.pool_max,
            timeout_acquire_ms: value.timeout_acquire_ms,
        }
    }
}

impl Default for PumpupSummaryConfig {
    fn default() -> Self {
        Self {
            active: ConfigValue::value(false),
            mode: ConfigValue::default(),
            connection_string: ConfigValue::default(),
            pool_min: ConfigValue::default(),
            pool_max: ConfigValue::default(),
            timeout_acquire_ms: ConfigValue::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PumpupTwapConfig {
    pub active: ConfigValue,
    pub mode: ConfigValue,

    pub connection_string: ConfigValue,
    pub pool_min: ConfigValue,
    pub pool_max: ConfigValue,
    pub timeout_acquire_ms: ConfigValue,
}

impl From<PumpupTwapConfig> for PostgresConfig {
    fn from(value: PumpupTwapConfig) -> Self {
        Self {
            connection_string: value.connection_string,
            pool_min: value.pool_min,
            pool_max: value.pool_max,
            timeout_acquire_ms: value.timeout_acquire_ms,
        }
    }
}

impl Default for PumpupTwapConfig {
    fn default() -> Self {
        Self {
            active: ConfigValue::value(false),
            mode: ConfigValue::default(),
            connection_string: ConfigValue::default(),
            pool_min: ConfigValue::default(),
            pool_max: ConfigValue::default(),
            timeout_acquire_ms: ConfigValue::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PumpupUsdConfig {
    pub active: ConfigValue,
    pub mode: ConfigValue,

    pub connection_string: ConfigValue,
    pub pool_min: ConfigValue,
    pub pool_max: ConfigValue,
    pub timeout_acquire_ms: ConfigValue,
}

impl From<PumpupUsdConfig> for PostgresConfig {
    fn from(value: PumpupUsdConfig) -> Self {
        Self {
            connection_string: value.connection_string,
            pool_min: value.pool_min,
            pool_max: value.pool_max,
            timeout_acquire_ms: value.timeout_acquire_ms,
        }
    }
}

impl Default for PumpupUsdConfig {
    fn default() -> Self {
        Self {
            active: ConfigValue::value(false),
            mode: ConfigValue::default(),
            connection_string: ConfigValue::default(),
            pool_min: ConfigValue::default(),
            pool_max: ConfigValue::default(),
            timeout_acquire_ms: ConfigValue::default(),
        }
    }
}
