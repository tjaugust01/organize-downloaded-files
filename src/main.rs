use std::{fs, thread};
use notify::{Event, RecursiveMode, Result, Watcher, EventKind};
use std::sync::mpsc;
use std::path::{Path, PathBuf};
use std::time::Duration;

const IMAGE_PATH: &str = "C:\\Users\\xkrys\\Pictures\\";
const TEXT_PATH: &str = "C:\\Users\\xkrys\\Documents\\";
const AUDIO_PATH: &str = "C:\\Users\\xkrys\\Music\\";
const VIDEO_PATH: &str = "C:\\Users\\xkrys\\Videos\\";
const ARCHIVE_PATH: &str = "C:\\Users\\xkrys\\Archive\\";
const EXECUTABLE_PATH: &str = "C:\\Users\\xkrys\\Desktop\\";
const OTHER_PATH: &str = "C:\\Users\\xkrys\\Downloads\\";

enum FileType {
    Image,
    Text,
    Audio,
    Video,
    Archive,
    Executable,
    Other,
    Invalid,
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

                            print!("File {:?}", path.display());
                            let file_destionation = set_file_destination(&path);
                            println!(" -> {:?}", file_destionation);
                            fs::rename(path, file_destionation)?;
                        }
                    }
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

fn set_file_destination(path: &PathBuf) -> PathBuf {
    let file_type = match_file_type(path);
    let file_name = path.file_name().unwrap();
    
    let destination_path = match file_type {
        FileType::Image => Path::new(IMAGE_PATH),
        FileType::Text => Path::new(TEXT_PATH),
        FileType::Audio => Path::new(AUDIO_PATH),
        FileType::Video => Path::new(VIDEO_PATH),
        FileType::Archive => Path::new(ARCHIVE_PATH),
        FileType::Executable => Path::new(EXECUTABLE_PATH),
        FileType::Other => Path::new(OTHER_PATH),
        FileType::Invalid => panic!("Invalid file type"),
    };
    destination_path.join(file_name)
}

fn match_file_type(path: &Path) -> FileType {
    let file_extension = path.extension();

    let file_type = match file_extension {
        Some(ext) => {
            match ext.to_str() {
                Some(ext) => {
                    match ext.to_lowercase().as_str() {
                        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" => FileType::Image,
                        "doc" | "docx" | "pdf" | "txt" | "odt" => FileType::Text,
                        "mp3" | "wav" => FileType::Audio,
                        "mp4" | "avi" | "mkv" | "mov" => FileType::Video,
                        "zip" | "rar" | "7z" | "tar" | "gz" => FileType::Archive,
                        "exe" | "msi" => FileType::Executable,
                        _ => FileType::Other,
                    }
                }
                None => FileType::Invalid
            }
        }
        None => FileType::Invalid
    };
    file_type
}