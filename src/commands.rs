use std::{
    fs,
    io::Write,
    path::Path,
    process::{Command, Output},
};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use crate::error::Error;

static HOOKS: &'static [u8] = include_bytes!("../assets/hook_template.sh");
static CAPTAIN_HOOK: &'static str = include_str!("../assets/captain_hook.sh");

fn git<S: AsRef<std::ffi::OsStr>>(args: &[S]) -> Result<Output, Error> {
    Ok(Command::new("git").args(args).output()?)
}

pub fn add(file_name: &str, cmd: &str) -> Result<(), Error> {
    let mut file = if Path::new(&file_name).exists() {
        fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(&file_name)?
    } else {
        let mut file = fs::File::create(&file_name)?;
        file.write(&HOOKS)?;
        if cfg!(unix) {
            let mut permissions = file.metadata()?.permissions();
            permissions.set_mode(0o0755);
            file.set_permissions(permissions)?;
        }

        file
    };

    Ok(writeln!(file, "{}", cmd)?)
}

pub fn install(dir: &str) -> Result<(), Error> {
    if git(&["rev-parse"])?.status.code() != Some(0) {
        return Ok(());
    }

    let p = Path::new(dir);
    fs::create_dir_all(p.join("_"))?;
    let mut gitignore = fs::File::create(p.join("_/.gitignore"))?;
    gitignore.write(b"*")?;

    let mut script = fs::File::create(p.join("_/captain_hook.sh"))?;
    write!(script, "{}", String::from(CAPTAIN_HOOK))?;
    if cfg!(unix) {
        let mut permissions = script.metadata()?.permissions();
        permissions.set_mode(0o0755);
        script.set_permissions(permissions)?;
    }

    git(&["config", "core.hooksPath", dir]).map(|_| ())
}

pub fn uninstall() -> Result<(), Error> {
    git(&["config", "--unset", "core.hooksPath"]).map(|_| ())
}
