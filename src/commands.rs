use std::{
    fs,
    io::Write,
    path::Path,
    process::{Command, Output},
};

#[cfg(not(target_os = "windows"))]
use std::os::unix::fs::PermissionsExt;

static HOOKS: &'static [u8] = include_bytes!("../assets/hook_template.sh");
static CAPTAIN_HOOK: &'static str = include_str!("../assets/captain_hook.sh");

fn git<S: AsRef<std::ffi::OsStr>>(args: &[S]) -> Output {
    let child = Command::new("git").args(args).spawn().expect("msg: &str");
    let out = child.wait_with_output().expect("msg: &str");
    out
}

pub fn add(file_name: &str, cmd: &str) {
    let mut file = if Path::new(&file_name).exists() {
        fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(&file_name)
            .unwrap()
    } else {
        let mut file = fs::File::create(&file_name).unwrap();
        file.write(&HOOKS).unwrap();
        if cfg!(not(target_os = "windows")) {
            let mut permissions = file.metadata().unwrap().permissions();
            permissions.set_mode(0o0755);
            file.set_permissions(permissions).unwrap();
        }

        file
    };

    writeln!(file, "{}", cmd).unwrap();
}

pub fn install(dir: &str) {
    if git(&["rev-parse"]).status.code() != Some(0) {
        return;
    }

    let p = Path::new(dir);
    fs::create_dir_all(p.join("_")).unwrap();
    let mut gitignore = fs::File::create(p.join("_/.gitignore")).unwrap();
    gitignore.write(b"*").unwrap();

    let mut script = fs::File::create(p.join("_/captain_hook.sh")).unwrap();
    write!(script, "{}", String::from(CAPTAIN_HOOK)).unwrap();
    if cfg!(not(target_os = "windows")) {
        let mut permissions = script.metadata().unwrap().permissions();
        permissions.set_mode(0o0755);
        script.set_permissions(permissions).unwrap();
    }

    git(&["config", "core.hooksPath", dir]);
}

pub fn uninstall() {
    git(&["config", "--unset", "core.hooksPath"]);
}
