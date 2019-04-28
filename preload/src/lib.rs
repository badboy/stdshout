use std::env;
use std::ffi::CString;
use std::fs::read_link;
use std::io::{stdin, stdout};
use std::os::unix::io::AsRawFd;

use ctor::ctor;
use nix::unistd::{close, dup, dup2, execv, fork, ForkResult, pipe};

#[ctor]
fn reopen_stdout() {
    let stdshout = match env::var("STDSHOUT_EXE") {
        Ok(var) => {
            if var.len() > 0 {
                var
            } else {
                eprintln!("Can't find stdshout exe. Set STDSHOUT_EXE to the full path.");
                return;
            }
        }
        _ => {
            eprintln!("Can't find stdshout exe. Set STDSHOUT_EXE to the full path.");
            return;
        }
    };

    let linkname = match read_link("/proc/self/exe") {
        Err(_) => {
            eprintln!("Can't read /proc/self/exe");
            return;
        }
        Ok(path) => format!("{}", path.display())
    };

    if linkname == stdshout {
        // We are stdshout already.
        return;
    }

    let stdout = stdout();
    let old_stdout = dup(stdout.as_raw_fd()).unwrap();

    let (read, write) = match pipe() {
        Ok((read, write)) => (read, write),
        Err(_) => {
            eprintln!("Can't open pipe");
            return;
        }
    };

    if let Err(_) = dup2(write, stdout.as_raw_fd()) {
        eprintln!("Can't dup write fd");
        return;
    }

    let args = [CString::new("stdshout").unwrap()];
    let path = CString::new(stdshout).unwrap();

    match fork() {
        Err(_) => {
            eprintln!("Can't fork");
            return;
        },
        Ok(ForkResult::Child) => {
            let _ = close(write);

            let _ = dup2(read, stdin().as_raw_fd());
            let _ = dup2(old_stdout, stdout.as_raw_fd());

            let _ = execv(&path, &args[..]);
            eprintln!("Something went wrong");
        },
        Ok(ForkResult::Parent { .. }) => {
            close(read).unwrap();
        }
    }

}
