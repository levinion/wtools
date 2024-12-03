#[derive(clap::Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Cmds,
}

#[derive(clap::Subcommand)]
pub enum Cmds {
    Window {
        #[command(subcommand)]
        cmd: WindowCli,
    },
    Desktop {
        #[command(subcommand)]
        cmd: DesktopCli,
    },
    System {
        #[command(subcommand)]
        cmd: SystemCli,
    },
}

#[derive(clap::Subcommand)]
pub enum WindowCli {
    Maximize,
    Minimize,
    Close,
    Fullscreen,
    Restore,
    MaximizeOrRestore,
}

#[derive(clap::Subcommand)]
#[allow(clippy::enum_variant_names)]
pub enum DesktopCli {
    SwitchTo { index: u32 },
    SwitchLeft,
    SwitchRight,
}

#[derive(clap::Subcommand)]
pub enum SystemCli {
    Lock,
}
