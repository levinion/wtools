mod cmd;

use clap::Parser;
use cmd::{Cli, Cmds, DesktopCli, SystemCli, WindowCli};
use desktop::{switch_left, switch_right, switch_to};
use wtools::*;

fn main() {
    let cli = Cli::parse();
    let window = window::Window::from_foreground();
    match cli.cmd {
        Cmds::Window { cmd } => match cmd {
            WindowCli::Maximize => {
                window.maximize();
            }
            WindowCli::Minimize => {
                window.minimize();
            }
            WindowCli::Close => {
                window.close();
            }
            WindowCli::Fullscreen => {
                window.fullscreen();
            }
            WindowCli::Restore => {
                window.restore();
            }
            WindowCli::MaximizeOrRestore => {
                window.maximize_or_restore();
            }
        },
        Cmds::Desktop { cmd } => match cmd {
            DesktopCli::SwitchTo { index } => {
                switch_to(index);
            }
            DesktopCli::SwitchLeft => {
                switch_left();
            }
            DesktopCli::SwitchRight => {
                switch_right();
            }
        },
        Cmds::System { cmd } => match cmd {
            SystemCli::Lock => {
                wtools::system::lock();
            }
        },
    }
}
