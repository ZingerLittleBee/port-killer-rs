use std::{
    env,
    io::{self, Error, ErrorKind, Write},
    process::{Command, Output, Stdio},
};

struct Win;
struct Unix;

trait Killer {
    fn get_pid(&self, port: u16) -> Result<Vec<u32>, Error>;
    fn kill(&self, pid: Vec<u32>) -> Result<bool, Error>;
}

impl Killer for Win {
    fn get_pid(&self, port: u16) -> Result<Vec<u32>, Error> {
        todo!()
    }
    fn kill(&self, pid: Vec<u32>) -> Result<bool, Error> {
        todo!()
    }
}

impl Killer for Unix {
    fn get_pid(&self, port: u16) -> Result<Vec<u32>, io::Error> {
        let lsof_child = Command::new("lsof")
            .arg(format!("-i:{}", port))
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to spawn lsof, may be you should install `lsof` first");
        let mut awk_child = Command::new("awk")
            .arg("NR > 1 { print $2 }")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        let awk_child_stdin = awk_child.stdin.as_mut().unwrap();
        let lsof_stdout = lsof_child
            .wait_with_output()
            .expect("Failed to wait for lsof")
            .stdout;
        awk_child_stdin
            .write_all(&lsof_stdout[..])
            .expect("Failed to write to awk stdin");
        drop(awk_child_stdin);
        let awk_output = awk_child
            .wait_with_output()
            .expect("Failed to wait for awk");
        if awk_output.status.success() {
            let res_str = String::from_utf8(awk_output.stdout).expect("Failed to parse awk output");
            let mut res_vec: Vec<u32> = res_str
                .split('\n')
                .filter_map(|s| s.parse::<u32>().ok())
                .collect();
            // remove dup values
            res_vec.sort();
            res_vec.dedup();
            Ok(res_vec)
        } else {
            Err(io::Error::new(
                ErrorKind::UnexpectedEof,
                String::from_utf8(awk_output.stderr).unwrap(),
            ))
        }
    }
    fn kill(&self, pid: Vec<u32>) -> Result<bool, Error> {
        let pid_str = pid.iter().map(|&p| p.to_string()).collect::<Vec<String>>();
        let output = Command::new("kill")
            .arg("-9")
            .args(&pid_str)
            .output()
            .expect("failed to execute process");
        return Ok(output.status.success());
    }
}

#[cfg(test)]
mod killer_tests {
    use port_selector::random_free_port;

    use super::*;

    #[test]
    fn get_pid_it_works() {
        let killer = if cfg!(target_os = "windows") {
            Box::new(Win) as Box<dyn Killer>
        } else {
            Box::new(Unix) as Box<dyn Killer>
        };
        let free_port = random_free_port().unwrap();
        let pids = killer.get_pid(free_port).unwrap();
        assert_eq!(pids, killer.get_pid(free_port).unwrap());
    }
}
