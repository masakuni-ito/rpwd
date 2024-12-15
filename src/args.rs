use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(
        short = 'L',
        long,
        help = "Display the logical current working directory"
    )]
    pub logical: bool,

    #[arg(
        short = 'P',
        long,
        help = "Display the physical current working directory"
    )]
    pub physical: bool,

    #[arg(short, long, help = "Divide path components with spaces")]
    pub divide: bool,

    #[arg(short, long, help = "Display the path with colored components")]
    pub color: bool,

    #[arg(short, long, help = "Display the path in stairs format")]
    pub stairs: bool,
}
