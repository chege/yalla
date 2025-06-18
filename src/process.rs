use std::process::{Command as Proc, ExitStatus};

use anyhow::anyhow;

pub fn execute(cmd: &str) -> anyhow::Result<ExitStatus> {
    let mut parts = shlex::split(cmd).ok_or_else(|| anyhow!("Failed to parse command: {cmd}"))?;
    if parts.is_empty() {
        return Err(anyhow!("Empty command"));
    }
    let program = parts.remove(0);
    let status = Proc::new(&program).args(&parts).status()?;
    if status.success() {
        Ok(status)
    } else {
        Err(anyhow!(status))
    }
}

pub fn exit_code(status: ExitStatus) -> i32 {
    if let Some(code) = status.code() {
        return code;
    }
    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt;
        if let Some(sig) = status.signal() {
            return 128 + sig;
        }
    }
    1
}
