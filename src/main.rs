extern crate nix;


fn main() {
    // let fd = nix::fcntl::open("/dev/ptmx", nix::fcntl::OFlag::O_RDWR | nix::fcntl::OFlag::O_NOCTTY, nix::sys::stat::Mode::S_IRUSR);

    let fd = match nix::pty::posix_openpt(nix::fcntl::OFlag::O_RDWR | nix::fcntl::OFlag::O_NOCTTY) {
        Ok(fd) => { fd },
        Err(why) => { panic!(why.to_string()) },
    };

    match nix::pty::grantpt(&fd) {
        Ok(_) => {  },
        Err(why) => { panic!(why.to_string()) },
    }

    match nix::pty::unlockpt(&fd) {
        Ok(_) => {  },
        Err(why) => { panic!(why.to_string()) },
    }

    let ptsname = match nix::pty::ptsname_r(&fd) {
        Ok(s) => { s },
        Err(why) => { panic!(why.to_string()) },
    };

    println!("{}", ptsname);


    // if .is_ok() {
    //     perror("grantpt");
    //     exit(1);
    // }
}
