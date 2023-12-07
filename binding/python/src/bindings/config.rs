use pyo3::{prelude::*, types::PyDict};
use raftify::{Config, RaftConfig};

#[derive(Clone)]
#[pyclass(name = "Config")]
pub struct PyConfig {
    raft_config: RaftConfig,
    #[pyo3(get, set)]
    log_dir: String,
    #[pyo3(get, set)]
    compacted_log_dir: String,
    #[pyo3(get, set)]
    compacted_logs_size_threshold: i32,
    #[pyo3(get, set)]
    max_retry_cnt: i32,
    #[pyo3(get, set)]
    message_timeout: f32,
    #[pyo3(get, set)]
    snapshot_interval: f32,
    #[pyo3(get, set)]
    tick_interval: f32,
    #[pyo3(get, set)]
    lmdb_map_size: i32,
    #[pyo3(get, set)]
    cluster_id: String,
    #[pyo3(get, set)]
    terminate_on_remove: bool,
}

#[pymethods]
impl PyConfig {
    #[new]
    fn new(kwargs: &PyDict) -> PyResult<Self> {
        let mut raft_config = RaftConfig::default();
        raft_config.applied = kwargs.get_item("applied")?.unwrap().extract::<u64>()?;
        raft_config.batch_append = kwargs
            .get_item("batch_append")?
            .unwrap()
            .extract::<bool>()?;
        raft_config.check_quorum = kwargs
            .get_item("check_quorum")?
            .unwrap()
            .extract::<bool>()?;
        raft_config.election_tick = kwargs
            .get_item("election_tick")?
            .unwrap()
            .extract::<usize>()?;
        raft_config.heartbeat_tick = kwargs
            .get_item("heartbeat_tick")?
            .unwrap()
            .extract::<usize>()?;
        raft_config.id = kwargs.get_item("id")?.unwrap().extract::<u64>()?;
        raft_config.max_committed_size_per_ready = kwargs
            .get_item("max_committed_size_per_ready")?
            .unwrap()
            .extract::<u64>()?;
        raft_config.max_inflight_msgs = kwargs
            .get_item("max_inflight_msgs")?
            .unwrap()
            .extract::<usize>()?;
        raft_config.max_size_per_msg = kwargs
            .get_item("max_size_per_msg")?
            .unwrap()
            .extract::<u64>()?;
        raft_config.max_uncommitted_size = kwargs
            .get_item("max_uncommitted_size")?
            .unwrap()
            .extract::<u64>()?;
        raft_config.max_election_tick = kwargs
            .get_item("max_election_tick")?
            .unwrap()
            .extract::<usize>()?;
        raft_config.min_election_tick = kwargs
            .get_item("min_election_tick")?
            .unwrap()
            .extract::<usize>()?;
        raft_config.pre_vote = kwargs.get_item("pre_vote")?.unwrap().extract::<bool>()?;
        raft_config.priority = kwargs.get_item("priority")?.unwrap().extract::<i64>()?;
        raft_config.skip_bcast_commit = kwargs
            .get_item("skip_bcast_commit")?
            .unwrap()
            .extract::<bool>()?;
        let read_only_option = kwargs
            .get_item("read_only_option")?
            .unwrap()
            .extract::<String>()?;

        raft_config.read_only_option = match read_only_option.as_str() {
            "Safe" => raftify::ReadOnlyOption::Safe,
            "LeaseBased" => raftify::ReadOnlyOption::LeaseBased,
            _ => unreachable!(),
        };

        let log_dir = kwargs.get_item("log_dir")?.unwrap().extract::<String>()?;
        let compacted_log_dir = kwargs
            .get_item("compacted_log_dir")?
            .unwrap()
            .extract::<String>()?;
        let compacted_logs_size_threshold = kwargs
            .get_item("compacted_logs_size_threshold")?
            .unwrap()
            .extract::<i32>()?;
        let max_retry_cnt = kwargs
            .get_item("max_retry_cnt")?
            .unwrap()
            .extract::<i32>()?;
        let message_timeout = kwargs
            .get_item("message_timeout")?
            .unwrap()
            .extract::<f32>()?;
        let snapshot_interval = kwargs
            .get_item("snapshot_interval")?
            .unwrap()
            .extract::<f32>()?;
        let tick_interval = kwargs
            .get_item("tick_interval")?
            .unwrap()
            .extract::<f32>()?;
        let lmdb_map_size = kwargs
            .get_item("lmdb_map_size")?
            .unwrap()
            .extract::<i32>()?;
        let cluster_id = kwargs
            .get_item("cluster_id")?
            .unwrap()
            .extract::<String>()?;
        let terminate_on_remove = kwargs
            .get_item("terminate_on_remove")?
            .unwrap()
            .extract::<bool>()?;

        Ok(Self {
            raft_config,
            log_dir,
            compacted_log_dir,
            compacted_logs_size_threshold,
            max_retry_cnt,
            message_timeout,
            snapshot_interval,
            tick_interval,
            lmdb_map_size,
            cluster_id,
            terminate_on_remove,
        })
    }
}

impl From<PyConfig> for Config {
    fn from(py_config: PyConfig) -> Self {
        Self {
            raft_config: py_config.raft_config,
            log_dir: py_config.log_dir,
            compacted_log_dir: py_config.compacted_log_dir,
            compacted_logs_size_threshold: py_config.compacted_logs_size_threshold,
            max_retry_cnt: py_config.max_retry_cnt,
            message_timeout: py_config.message_timeout,
            snapshot_interval: py_config.snapshot_interval,
            tick_interval: py_config.tick_interval,
            lmdb_map_size: py_config.lmdb_map_size,
            cluster_id: py_config.cluster_id,
            terminate_on_remove: py_config.terminate_on_remove,
        }
    }
}