extern crate nix;
extern crate ncurses;
extern crate rlua;


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


    ncurses::initscr();

    /* Print to the back buffer. */
    ncurses::printw("Hello, world!");

    /* Print some unicode(Chinese) string. */
    // printw("Great Firewall dislike VPN protocol.\nGFW 不喜欢 VPN 协议。");

    /* Update the screen. */
    ncurses::refresh();

    /* Wait for a key press. */
    ncurses::getch();

    /* Terminate ncurses. */
    ncurses::endwin();

    let lua = rlua::Lua::new();
    match lua.eval::<_, ()>(
        r#"
        local t = {
            hoge = 'hello'
        }
        print(t.hoge)
        "#,
        None,
    )  {
        Ok(_) => {  },
        Err(why) => { panic!(why.to_string()) },
    }
}
