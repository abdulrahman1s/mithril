extern crate mithril;
extern crate config;

use mithril::stratum::stratum_data::{PoolConfig};
use mithril::stratum::stratum::{StratumClient, StratumAction};
use mithril::worker::worker_pool;
use mithril::worker::worker_pool::{WorkerConfig};
use mithril::metric::metric;
use mithril::metric::metric::{MetricConfig};
use mithril::cryptonight::hash;
use mithril::byte_string;
use std::sync::mpsc::{channel};
use std::path::Path;
use config::{Config, ConfigError, File};

const CONFIG_FILE_NAME : &'static str = "config.toml";

fn main() {

    sanity_check();

    //Read config
    let config = read_config().unwrap();
    let pool_conf = pool_config(&config).unwrap();
    let worker_conf = worker_config(&config).unwrap();
    let metric_conf = metric_config(&config).unwrap();

    //Stratum start
    let (stratum_tx, stratum_rx) = channel();

    println!("Doing client login");
    let mut client = StratumClient::new(pool_conf, vec![stratum_tx]);
    client.login();

    let share_tx = client.new_cmd_channel().unwrap();
    let (metric_tx, metric_rx) = channel();

    metric::start(metric_conf.clone(), metric_rx);

    //worker pool start
    let pool = &worker_pool::start(worker_conf, share_tx, metric_conf.resolution, metric_tx);

    loop {
        let received = stratum_rx.recv();
        if received.is_err() {
            println!("lost connection to stratum client: {:?}", received);
            return
        }
        match received.unwrap() {
            StratumAction::Job{miner_id, blob, job_id, target} => {
                pool.job_change(miner_id, blob, job_id, target);
            },
            StratumAction::Error{err} => {
                println!("received stratum error: {}", err);
            }
            StratumAction::Ok => {
                println!("received stratum ok");
            }
        }
    }
}

fn pool_config(conf: &Config) -> Result<PoolConfig, ConfigError> {
    let pool_address = conf.get_str("pool.pool_address")?;
    let wallet_address = conf.get_str("pool.wallet_address")?;
    let pool_password = conf.get_str("pool.pool_password")?;
    return Ok(PoolConfig{pool_address, wallet_address, pool_password});
}

fn worker_config(conf: &Config) -> Result<WorkerConfig, ConfigError> {
    let num_threads = conf.get_int("worker.num_threads")?;
    if num_threads <= 0 {
        return Err(ConfigError::Message("num_threads hat to be > 0".to_string()));
    }
    return Ok(WorkerConfig{num_threads: num_threads as u64});
}

fn metric_config(conf: &Config) -> Result<MetricConfig, ConfigError> {
    let enabled = conf.get_bool("metric.enabled")?;
    if enabled {
        let resolution = get_u64_no_zero(conf, "metric.resolution")?;
        let sample_interval_seconds = get_u64_no_zero(conf, "metric.sample_interval_seconds")?;
        let report_file = conf.get_str("metric.report_file")?;
        return Ok(MetricConfig{enabled, resolution, sample_interval_seconds, report_file});
    } else {
        return Ok(MetricConfig{enabled: false, resolution: std::u64::MAX,
                               sample_interval_seconds: std::u64::MAX, report_file: "/dev/null".to_string()});
    }
}

fn get_u64_no_zero(conf: &Config, field: &str) -> Result<u64, ConfigError> {
    let val = conf.get_int(field)?;
    if val <= 0 {
        return Err(ConfigError::Message(format!("{} has to be > 0", field)));
    }
    return Ok(val as u64);
}

fn read_config() -> Result<Config, ConfigError> {
    let cwd_path = &format!("{}{}", "./", CONFIG_FILE_NAME);
    let cwd_config_file = Path::new(cwd_path);
    if cwd_config_file.exists() {
        let mut conf = Config::default();
        conf.merge(File::with_name(CONFIG_FILE_NAME))?;
        return Ok(conf);
    }
    return Err(ConfigError::Message("config file not found".to_string()));
}

fn sanity_check() {
    let result0 = hash::hash(&byte_string::string_to_u8_array(""));
    let result1 = hash::hash(&b"This is a test"[0..]);
    if result0 != "eb14e8a833fac6fe9a43b57b336789c46ffe93f2868452240720607b14387e11" ||
       result1 != "a084f01d1437a09c6985401b60d43554ae105802c5f5d8a9b3253649c0be6605" {
        panic!("hash sanity check failed, please report this at https://github.com/Ragnaroek/mithril/issues");
    }
}
