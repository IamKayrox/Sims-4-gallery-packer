use std::env;
use std::fs;
use std::fs::ReadDir;
use std::io::stdin;
use std::io::ErrorKind;
use std::path::PathBuf;
use crate::constants;
use crate::tray_files::TrayFolderContent;
use crate::tray_files::TrayItem;
use crate::tray_files::TrayItemFile;
use colored::Colorize;
use directories::UserDirs;

pub fn get_tray_folder() -> Option<ReadDir> {
    match UserDirs::new() {
        Some(user_dirs) => {
            match user_dirs.document_dir() {
                Some(document_dir) => {
                    let tray_folder = document_dir.join("Electronic Arts").join("The Sims 4").join("Tray");
                    match fs::read_dir(tray_folder) {
                        Ok(items) => {
                            return Some(items);
                        },
                        Err(e) => {
                            if e.kind() == ErrorKind::NotFound {
                                print_error(String::from("Tray folder doesn't exists!"));
                            } else {
                                print_error(String::from("Unable to read the tray folder!"));
                            }
                            return None;
                        }
                    }
                },
                None => {
                    print_error(String::from("Couldn't find the document directory"));
                    return None;
                }
            }
        },
        None => {
            print_error(String::from("Couldn't find user directories"));
            return None;
        }
    }
}

pub fn prepare_output_folder() -> Option<PathBuf> {
    match env::current_dir() {
        Ok(current_dir) => {
            let output_path = current_dir.join("output");
            match fs::create_dir(&output_path) {
                Ok(_) => {
                    return Some(output_path);
                },
                Err(e) => {
                    if e.kind() == ErrorKind::AlreadyExists {
                        match fs::remove_dir_all(&output_path) {
                            Ok(_) => {
                                match fs::create_dir(&output_path) {
                                    Ok(_) => {
                                        return Some(output_path);
                                    },
                                    Err(_) => {
                                        print_error(String::from("Cound't create the output directory"));
                                        return None;
                                    }
                                }
                            },
                            Err(_) => {
                                print_warning(String::from("Output folder already exists and couldn't be removed"));
                                print_info(String::from("Process can continue but it may fail. Do you want to continue? [y/N]"));
                                let mut input = String::new();
                                loop {
                                    match stdin().read_line(&mut input) {
                                        Ok(_) => {
                                            let normalized_input = input.trim().to_lowercase();
                                            let normalized_input = normalized_input.as_str();
                                            match normalized_input {
                                                "y" | "yes" => {
                                                    return Some(output_path);
                                                },
                                                "n" | "no" | "" => {
                                                    return None;
                                                },
                                                _ => {
                                                    print_info(String::from("Invalid input. Do you want to continue? [y/N]"));
                                                }
                                            }
                                        },
                                        Err(_) => {
                                            print_error(String::from("Couldn't read input."));
                                            return None;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        print_error(String::from("Cound't create the output directory"));
                        return None;
                    }
                }
            }
        },
        Err(_) => {
            print_error(String::from("Cound't find the current directory"));
            return None;
        }
    }
}

pub fn read_tray_files(content: ReadDir) -> TrayFolderContent {
    let mut trayitem_files: Vec<TrayItem> = Vec::new();
    let mut unknown_files: Vec<TrayItemFile> = Vec::new();
    let mut sgi_files: Vec<TrayItemFile> = Vec::new();
    for item in content {
        match item {
            Ok(item) => {
                let path = item.path();
                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        if let Some(extension) = extension.to_str() {
                            match extension {
                                constants::TRAYITEM_EXT => {
                                    if let Some(trayitem) = TrayItem::new(&path) {
                                        trayitem_files.push(trayitem);
                                    }
                                },
                                constants::HOUSEHOLDBINARY_EXT |
                                constants::HHI_EXT |
                                constants::BLUEPRINT_EXT |
                                constants::BPI_EXT |
                                constants::ROOM_EXT |
                                constants::RMI_EXT  => {
                                    if let Some(unknown_file) = TrayItemFile::new(&path) {
                                        unknown_files.push(unknown_file);
                                    }
                                }
                                constants::SGI_EXT => {
                                    if let Some(sgi_file) = TrayItemFile::new(&path) {
                                        sgi_files.push(sgi_file);
                                    }
                                },
                                _ => {
                                    print_warning(String::from("Unknown extension, skipping file."));
                                }
                            }
                        } else {
                            print_warning(String::from("Couldn't parse file extension, skipping!"));
                        }
                    } else {
                        print_warning(format!("Couldn't get file extension, skipping! ({})", path.display()));
                    }
                }
            },
            Err(_) => {
                print_warning(String::from("Couldn't read item in tray folder, skipping!"));
            }
            
        }
    }
    return TrayFolderContent {
        trayitem_files,
        unknown_files,
        sgi_files,
    }
}

pub fn extract_id_and_type(path: &PathBuf) -> Option<(u32, u64)> {
    if let Some(stem) = path.file_stem() {
        if let Some(stem) = stem.to_str() {
            let parts: Vec<&str> = stem.split("!").collect();
            if parts.len() == 2 {
                let trimmed = parts[1][4..].to_string();
                let flat_id = format!("00{}", trimmed);
                match u64::from_str_radix(&flat_id, 16) {
                    Ok(id) => {
                        match u32::from_str_radix(parts[0].trim_start_matches("0x"), 16) {
                            Ok(file_type) => {
                                return Some((file_type, id));
                            },
                            Err(_) => {
                                print_warning(format!("Couldn't parse file type from file name, skipping! ({})", path.display()));
                            }
                        }
                    },
                    Err(_) => {
                        print_warning(format!("Couldn't parse id from file name, skipping! ({})", path.display()));
                    }
                }
            } else {
                print_warning(format!("Invalid file name, skipping! ({})", path.display()));
            }
        } else {
            print_warning(format!("Couldn't parse file name, skipping! ({})", path.display()));
        }
    } else {
        print_warning(format!("Couldn't get file name, skipping! ({})", path.display()));
    }
    return None;
}

pub fn extract_string(content: &Vec<u8>, offset: usize, length: u8) -> String {
    let raw = content[offset..(offset + length as usize)].to_vec();
    let mut result = String::new();
    for byte in raw {
        result.push(byte as char);
    }
    return result;
}

pub fn preapare_output_folder_for_type(outputfolder: &PathBuf, filetype: u32) -> Option<PathBuf> {
    let folder_name: String;
    match filetype {
        constants::HOUSEHOLD_TRAYITEM_TYPE => {
            folder_name = String::from(constants::HOUSEHOLD_FOLDERNAME);
        },
        constants::PLOT_TRAYITEM_TYPE => {
            folder_name = String::from(constants::PLOT_FOLDERNAME);
        },
        constants::ROOM_TRAYITEM_TYPE => {
            folder_name = String::from(constants::ROOM_FOLDERNAME);
        },
        _ => {
            print_warning(String::from("Unknown gallery item type, skipping!"));
            return None;
        }
    }
    let target_folder = outputfolder.join(&folder_name);
    match fs::create_dir(&target_folder) {
        Ok(_) => {
            return Some(target_folder);
        },
        Err(e) => {
            if e.kind() == ErrorKind::AlreadyExists {
                return Some(target_folder);
            }
            print_error(format!("Unable to create output folder for {} items!", &folder_name));
            print_warning(format!("All {} items may be skipped!", &folder_name));
            return None;
        }
    }
}

pub fn print_success(message: String) {
    println!("{} {}", "[SUCCESS]".bright_green(), message);
}

pub fn print_warning(message: String) {
    println!("{} {}", "[WARNING]".yellow(), message);
}

pub fn print_error(message: String) {
    println!("{} {}", "[ERROR]".red(), message);
}

pub fn print_info(message: String) {
    println!("{} {}", "[INFO]".bright_blue(), message);
}

pub fn print_debug(message: String) {
    println!("{} {}", "[DEBUG]".bright_magenta(), message);
}