use std::env;
use std::error::Error;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use pathrs::flags::OpenFlags;
use pathrs::Root;

fn main() -> Result<(), Box<dyn Error>> {
    let kernel_release = fs::read_to_string("/proc/sys/kernel/osrelease")?;
    let root_path = env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| env::temp_dir().join(format!("pathrs-repro-{}", std::process::id())));

    eprintln!("kernel release: {}", kernel_release.trim());
    eprintln!("reproducer root: {}", root_path.display());

    fs::create_dir_all(&root_path)?;
    let root = Root::open(&root_path)?;
    eprintln!("Root::open succeeded");

    let parent = root.mkdir_all("containers/repro", &fs::Permissions::from_mode(0o750))?;
    eprintln!("Root::mkdir_all succeeded");

    let _directory = parent.reopen(OpenFlags::O_DIRECTORY)?;
    eprintln!("Handle::reopen succeeded");

    fs::remove_dir_all(&root_path)?;
    eprintln!("pathrs operations completed successfully");
    Ok(())
}
