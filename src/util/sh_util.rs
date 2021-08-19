use std::process::Command;
use crate::util::error::Error;

pub(crate) fn run(cmd: &str, args: &[&str]) -> Result<(), Error> {
    let mut command = Command::new(cmd);
    command.args(args);
    let mut child = command.spawn()?;
    let status = child.wait()?;
    if status.success() {
        Ok(())
    } else {
        match status.code() {
            None => { Err(Error::from("Process failed. No error code available.")) }
            Some(code) => {
                Err(Error::from(format!("Process failed with error code {}", code)))
            }
        }
    }
}