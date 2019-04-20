use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
pub struct Config {
    #[structopt(short = "w", long = "width", default_value = "640")]
    /// Render window width
    pub width: usize,
    #[structopt(short = "h", long = "height", default_value = "480")]
    /// Render window height
    pub height: usize,
}
