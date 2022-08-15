
use clap::{ Parser, Subcommand };


#[derive(Parser, Debug, Clone)]
#[clap(version, about)] //#[clap(author, version, about)]
pub struct MainArg {

    #[clap(subcommand)]
    pub mode: Mode,

}


#[derive(Subcommand, Debug, Clone)]
pub enum Mode {

    /// one message on one top
    #[clap(display_order = 1)]
    Single(SingleArg),

    /// one message on many top
    #[clap(display_order = 2)]
    Multiple(MultipleArg),

    /// many message on many top
    #[clap(display_order = 3)]
    Multiple2(Multiple2Arg),

    /// one long message on many top with newline
    #[clap(display_order = 4)]
    Long(LongArg),

    /// message on many top vertically
    #[clap(display_order = 5)]
    Vertical(VerticalArg),

    /// one message on one top like electric bulletin board
    #[clap(display_order = 6)]
    Wave(WaveArg),

    /// check if top_message can be executed normally
    #[clap(display_order = 7)]
    Check(CheckArg),

}



#[derive(Debug, clap::Args, Clone)]
#[clap(arg_required_else_help = true, version)]
pub struct SingleArg {

    #[clap(short, long, value_name = "STR", help = "message that appears on top", display_order = 1)]
    pub message: String,

    #[clap(short = '@', long, value_name = "INT", default_value = "1", help = "thread number", display_order = 2)]
    pub thread: usize,

    #[clap(short, long, value_name = "INT", default_value = "10", help = "display time(s)", display_order = 3)]
    pub time: usize,

    #[clap(long = "tmpdir", value_name = "STR", default_value = "./.tmp_(date_randomnumber_pid)", help = "tmp directory name", display_order = 4)]
    pub tmpdir: String,

}


#[derive(Debug, clap::Args, Clone)]
#[clap(arg_required_else_help = true, version)]
pub struct MultipleArg {

    #[clap(short, long, value_name = "STR", help = "message that appears on top", display_order = 1)]
    pub message: String,

    #[clap(short = '@', long, value_name = "INT", default_value = "1", help = "thread number", display_order = 2)]
    pub thread: usize,

    #[clap(short, long, value_name = "INT", default_value = "10", help = "display time(s)", display_order = 3)]
    pub time: usize,

    #[clap(long = "tmpdir", value_name = "STR", default_value = "./.tmp_(date_randomnumber_pid)", help = "tmp directory name", display_order = 4)]
    pub tmpdir: String,

}



#[derive(Debug, clap::Args, Clone)]
#[clap(arg_required_else_help = true, version)]
pub struct Multiple2Arg {

    #[clap(short, long, value_name = "\"STR STR STR ...\"", help = "message that appears on top\n[CAUTION] If more than one messge, \"\" or \'\' is necessary\n[CAUTION] number of thread used is automatically determined", value_parser, display_order = 1)]
    pub message: String,

    #[clap(short, long, value_name = "INT", default_value = "10", help = "display time(s)", display_order = 2)]
    pub time: usize,

    #[clap(long = "tmpdir", value_name = "STR", default_value = "./.tmp_(date_randomnumber_pid)", help = "tmp directory name", display_order = 3)]
    pub tmpdir: String,

}



#[derive(Debug, clap::Args, Clone)]
#[clap(arg_required_else_help = true, version)]
pub struct LongArg {

    #[clap(short, long, value_name = "STR", help = "one long message that appears on top\n[CAUTION] number of thread used is automatically determined", display_order = 1)]
    pub message: String,

    #[clap(short, long, value_name = "INT", default_value = "10", help = "display time(s)", display_order = 2)]
    pub time: usize,

    #[clap(short, long, value_name = "INT", default_value = "12", help = "characters per top", display_order = 3)]
    pub length: usize,

    #[clap(long = "tmpdir", value_name = "STR", default_value = "./.tmp_(date_randomnumber_pid)", help = "tmp directory name", display_order = 4)]
    pub tmpdir: String,

}



#[derive(Debug, clap::Args, Clone)]
#[clap(arg_required_else_help = true, version)]
pub struct VerticalArg {

    #[clap(short, long, value_name = "\"STR STR STR ...\"", help = "message that appears on top\n[CAUTION] If more than one messge, \"\" or \'\' is necessary\n[CAUTION] number of thread used is automatically determined", value_parser, display_order = 1)]
    pub message: String,

    #[clap(short, long, value_name = "INT", default_value = "10", help = "display time(s)", display_order = 2)]
    pub time: usize,

    #[clap(long = "tmpdir", value_name = "STR", default_value = "./.tmp_(date_randomnumber_pid)", help = "tmp directory name", display_order = 3)]
    pub tmpdir: String,

}


#[derive(Debug, clap::Args, Clone)]
#[clap(arg_required_else_help = true, version)]
pub struct WaveArg {

    #[clap(short, long, value_name = "STR", help = "one message on one top like electric bulletin board\n[CAUTION] number of thread used is automatically determined", display_order = 1)]
    pub message: String,

    #[clap(short = '@', long, value_name = "INT", default_value = "1", help = "thread numer", display_order = 2)]
    pub thread: usize,

    #[clap(short, long, value_name = "INT", default_value = "12", help = "characters per top", display_order = 3)]
    pub length: usize,

    #[clap(long = "tmpdir", value_name = "STR", default_value = "./.tmp_(date_randomnumber_pid)", help = "tmp directory name", display_order = 4)]
    pub tmpdir: String,

}



#[derive(Debug, clap::Args, Clone)]
#[clap(arg_required_else_help = true, version)]
pub struct CheckArg {

    #[clap(long, help = "check that top_message can be executed normally & tmp directory that top_message created", display_order = 1)]
    pub onlycheck: bool,

    #[clap(long, help = "check that top_message can be executed normally", display_order = 2)]
    pub onlyrustcheck: bool,

    #[clap(long, help = "check tmp directory that top_message created", display_order = 3)]
    pub onlydircheck: bool,

    #[clap(long, help = "check that top_message can be executed normally & \nREMOVE tmp dirctory that top_message created", display_order = 4)]
    pub rmcheck: bool,

}




pub fn arg() -> MainArg {

    MainArg::parse()

}

