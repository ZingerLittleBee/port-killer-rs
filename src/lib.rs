use std::io::Error;

use killer::{Killer, Unix, Win};

mod killer;

pub fn kill(port: u16) -> Result<bool, Error> {
    let killer = if cfg!(target_os = "windows") {
        Box::new(Win) as Box<dyn Killer>
    } else {
        Box::new(Unix) as Box<dyn Killer>
    };
    let pids = killer
        .get_pid(port)
        .unwrap_or_else(|e| panic!("Failed to get pids on port: {}, error: {}", port, e));
    Ok(killer
        .kill(pids)
        .unwrap_or_else(|e| panic!("Failed to kill process on port: {}, error: {}", port, e)))
}

pub fn kill_by_pids(pids: &[u32]) -> Result<bool, Error> {
    let killer = if cfg!(target_os = "windows") {
        Box::new(Win) as Box<dyn Killer>
    } else {
        Box::new(Unix) as Box<dyn Killer>
    };
    Ok(killer
        .kill(pids.to_vec())
        .unwrap_or_else(|e| panic!("Failed to kill process on pids: {:#?}, Error: {}", pids, e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_kill() {
        assert!(kill(5000).expect(""));
    }

    #[ignore]
    #[test]
    fn test_kill_by_pids() {
        assert!(kill_by_pids(&[56812]).expect(""));
    }
}
