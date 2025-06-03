use notify::{Event, RecursiveMode, Result, Watcher, EventKind};
use std::sync::mpsc;
use std::path::Path;

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel::<Result<Event>>();
    let mut watcher = notify::recommended_watcher(tx)?;
    let path=Path::new("../../../../../../mnt/c/Users/xkrys/Downloads/");
    watcher.watch(path, RecursiveMode::Recursive)?;
    for res in rx {
        match res {
            Ok(event) => {
                println!("{:?}", event);
                if let EventKind::Create(_) = event.kind {
                    for path in event.paths {
                        if path.is_file() {
                            println!("Neue Datei erstellt: {:?}", path);
                        }
                    }
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}