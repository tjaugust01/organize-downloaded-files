use std::{fs, thread};
use notify::{Event, RecursiveMode, Result, Watcher, EventKind};
use std::sync::mpsc;
use std::path::Path;
use std::time::Duration;
fn set_file_destionation(file_name: Option<&str>, file_extenstion: Option<&str>)
                         -> Option<&str> {
    let string_file_name = file_name?;
    let string_file_extenstion = file_extenstion?;
    let destionation_path = Path::new("C:\\Users\\xkrys\\Downloads\\");
    return destionation_path.to_str();
}
fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel::<Result<Event>>();
    let mut watcher = notify::recommended_watcher(tx)?;
    let path=Path::new(".\\..\\..\\Users\\xkrys\\Downloads\\");
    watcher.watch(path, RecursiveMode::Recursive)?;
    println!("Watching... {:?}",fs::canonicalize(path));

    for res in rx {
        match res {
            Ok(event) => {
                if let EventKind::Create(_) = event.kind {
                    for path in event.paths {
                        if path.is_file() {
                            let mut previous_size = 0;

                            loop {
                                let metadata = fs::metadata(&path)?;
                                let current_size = metadata.len();

                                if current_size > 0 && current_size == previous_size {
                                    break;
                                }

                                previous_size = current_size;
                                thread::sleep(Duration::from_secs(1));
                            }
                            let mut file_name= path.file_name().unwrap().to_str();
                            let mut file_extenstion= path.extension().unwrap().to_str();
                            
                            
                            if file_name.is_some() && file_extenstion.is_some() {
                                let file_destionation = set_file_destionation(file_name, file_extenstion);
                            }
                            println!("Filename{:?}", path.file_name().unwrap());
                            fs::rename(path, "test.png")?;
                        }
                    }
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}