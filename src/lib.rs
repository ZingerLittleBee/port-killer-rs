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
        .expect(format!("Failed to get pids on port: {}", port).as_str());
    println!("pids: {:#?}", pids);
    Ok(killer
        .kill(pids)
        .expect(format!("Failed to kill process on port: {}", port).as_str()))
}

pub fn kill_by_pids(pids: &[u32]) -> Result<bool, Error> {
    let killer = if cfg!(target_os = "windows") {
        Box::new(Win) as Box<dyn Killer>
    } else {
        Box::new(Unix) as Box<dyn Killer>
    };
    Ok(killer
        .kill(pids.to_vec())
        .expect(format!("Failed to kill process on pids: {:#?}", pids).as_str()))
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
