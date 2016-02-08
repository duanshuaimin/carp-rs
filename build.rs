extern crate gcc;

fn main() {
    gcc::Config::new()
        .file("src/bsd-getopt_long.c")
        .file("src/carp.c")
        .file("src/crypto-sha1.c")
        .file("src/daemonize.c")
        .file("src/fakesnprintf.c")
        .file("src/fillmac.c")
        .file("src/garp.c")
        .file("src/log.c")
        .file("src/mysnprintf.c")
        .file("src/spawn.c")
        .file("src/ucarp.c")
        .include("./")
        .compile("libcarp.a");
}
