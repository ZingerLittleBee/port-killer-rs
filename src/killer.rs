use std::{
    io::{self, Error, ErrorKind, Write},
    process::{Command, Stdio},
};

struct Win;
struct Unix;

trait Killer {
    fn get_pid(&self, port: u16) -> Result<Vec<u32>, Error>;
    fn kill(&self, pid: u32) -> Result<bool, Error>;
}

impl Killer for Win {
    fn get_pid(&self, port: u16) -> Result<Vec<u32>, Error> {
        todo!()
    }
    fn kill(&self, pid: u32) -> Result<bool, Error> {
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
            // TODO: remove dup port
            let res_vec: Vec<u32> = res_str
                .split('\n')
                .filter_map(|s| s.parse::<u32>().ok())
                .collect();
            Ok(res_vec)
        } else {
            Err(io::Error::new(
                ErrorKind::UnexpectedEof,
                String::from_utf8(awk_output.stderr).unwrap(),
            ))
        }
    }
    fn kill(&self, pid: u32) -> Result<bool, Error> {
        todo!()
    }
}

#[cfg(test)]
mod killer_tests {
    use super::*;

    #[test]
    fn get_pid_it_works() {
        let killer = Unix;
        println!("killer.get_pid(5000): {:#?}", killer.get_pid(5000).unwrap());
    }
}
