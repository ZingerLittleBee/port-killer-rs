use killer::{Killer, Unix, Win};

mod killer;

pub fn kill(port: u16) -> Result<bool, String> {
    let killer = if cfg!(target_os = "windows") {
        Box::new(Win) as Box<dyn Killer>
    } else {
        Box::new(Unix) as Box<dyn Killer>
    };

    match killer.get_pid(port) {
        Ok(pids) => kill_by_pids(&pids),
        Err(error) => Result::Err(format!("Failed to get pids on port: {}, error: {}", port, error)),
    }
}

pub fn kill_by_pids(pids: &[u32]) -> Result<bool, String> {
    let killer = if cfg!(target_os = "windows") {
        Box::new(Win) as Box<dyn Killer>
    } else {
        Box::new(Unix) as Box<dyn Killer>
    };
    match killer.kill(pids.to_vec()) {
        Ok(result) => Ok(result),
        Err(error) => Err(format!("Failed to kill process on pids: {:#?}, Error: {}", pids, error)),
    }
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
