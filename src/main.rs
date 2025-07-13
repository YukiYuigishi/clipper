use std::io::{self, Read};

use arboard::{Clipboard, SetExtWindows};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    secure: bool,

    #[arg(long, hide = true)]
    clear_clipboard_task: Option<String>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    //    if let Some(text_to_clear) = args.clear_clipboard_task {
    //        thread::sleep(Duration::from_secs(45));
    //
    //        let mut clipboard = Clipboard::new().unwrap();
    //        if let Ok(current_clipboard) = clipboard.get_text() {
    //            if current_clipboard == text_to_clear {
    //                clipboard.clear().unwrap();
    //            }
    //        }
    //        return Ok(());
    //    }

    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buffer = Vec::new();
    handle.read_to_end(&mut buffer)?;
    let utf8_input = String::from_utf8_lossy(&buffer).to_string();

    let mut clipboard = Clipboard::new().unwrap();

    if args.secure {
        let set = clipboard.set();
        set.exclude_from_history().text(utf8_input.clone()).unwrap();

    //        let current_exe = std::env::current_exe()?;
    //        let mut cmd = Command::new(current_exe);
    //        cmd.arg("--clear-clipboard-task").arg(utf8_input);
    //
    //        cmd.stdin(Stdio::null())
    //            .stdout(Stdio::null())
    //            .stderr(Stdio::null());
    //
    //        #[cfg(windows)]
    //        {
    //            use std::os::windows::process::CommandExt;
    //            const DETACHED_PROCESS: u32 = 0x00000008;
    //            const CREATE_NEW_PROCESS_GROUP: u32 = 0x00000200;
    //            cmd.creation_flags(DETACHED_PROCESS | CREATE_NEW_PROCESS_GROUP);
    //        }
    //
    //        cmd.spawn()
    //            .expect("Failed to spawn background process to clear clipboard.");
    } else {
        clipboard.set_text(utf8_input).unwrap();
    }

    Ok(())
}

