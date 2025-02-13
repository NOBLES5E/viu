use clap::{crate_description, crate_name, crate_version, Arg, Command};

mod app;
mod config;

use config::Config;

fn main() {
    let matches = Command::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg_required_else_help(true)
        .arg(
            Arg::new("FILE")
                .help("The images to be displayed. Set to - for standard input.")
                .multiple_values(true),
        )
        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .help("Output the name of the file before displaying"),
        )
        .arg(
            Arg::new("transparent")
                .short('t')
                .long("transparent")
                .help("Display transparent image with transparent background"),
        )
        .arg(
            Arg::new("once")
                .short('1')
                .long("once")
                .help("Only loop once through the animation"),
        )
        .arg(
            Arg::new("static")
                .short('s')
                .long("static")
                .help("Show only first frame of gif"),
        )
        .arg(
            Arg::new("x")
                .short('x')
                .takes_value(true)
                .help("X offset. Defaults to 0."),
        )
        .arg(
            Arg::new("y")
                .short('y')
                .takes_value(true)
                .help("Y offset. Defaults to 0."),
        )
        .arg(
            Arg::new("absolute-offset")
                .short('a')
                .long("absolute-offset")
                .takes_value(false)
                .help("Make the x and y offset be relative to the top left terminal corner. If false, the y offset is relative to the cursor's position. Defaults to true."),
        )
        .arg(
            Arg::new("width")
                .short('w')
                .long("width")
                .takes_value(true)
                .help("Resize the image to a provided width"),
        )
        .arg(
            Arg::new("height")
                .short('h')
                .long("height")
                .takes_value(true)
                .help("Resize the image to a provided height"),
        )
        .arg(
            Arg::new("recursive")
                .short('r')
                .long("recursive")
                .help("Recurse down directories if passed one"),
        )
        .arg(
            Arg::new("frames-per-second")
                .short('f')
                .long("frame-rate")
                .takes_value(true)
                .help("Play gif at the given frame rate"),
        )
        .arg(
            Arg::new("blocks")
                .short('b')
                .long("blocks")
                .takes_value(false)
                .help("Force block output"),
        )
        .get_matches();

    let conf = Config::new(&matches);

    if let Err(e) = app::run(conf) {
        eprintln!("{:?}", e);
        std::process::exit(1);
    }
}
