extern crate nix;


fn main() {
    // let fd = nix::fcntl::open("/dev/ptmx", nix::fcntl::OFlag::O_RDWR | nix::fcntl::OFlag::O_NOCTTY, nix::sys::stat::Mode::S_IRUSR);

    if fd.is_err() {
        eprintln!("error parsing header: {:?}", fd.err());
        std::process::exit(1);
    }

    match nix::pty::grantpt(fd) {
        Err(e) => {
            eprintln!("error parsing header: {:?}", e);
            std::process::exit(1);
        },
    }

    // if .is_ok() {
    //     perror("grantpt");
    //     exit(1);
    // }
}
