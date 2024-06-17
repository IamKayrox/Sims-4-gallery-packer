use std::{fs, path::PathBuf};

use colored::Colorize;

use crate::{constants, utils};

pub struct TrayFolderContent {
    pub trayitem_files: Vec<TrayItem>,
    pub unknown_files: Vec<TrayItemFile>,
    pub sgi_files: Vec<TrayItemFile>,
}

pub struct TrayItem {
    pub path: PathBuf,
    pub name: String,
    pub id: u64,
    pub file_type: u32,
    pub filename: String,
}

impl TrayItem {
    pub fn new(path: &PathBuf) -> Option<TrayItem> {
        if let Some((file_type, id)) = utils::extract_id_and_type(&path) {
            match fs::read(path) {
                Ok(content) => {
                    if let Some(file_name) = path.file_name() {
                        if let Some(file_name) = file_name.to_str() {
                            let name_size: u8;
                            let offset: usize;
                            match file_type {
                                constants::HOUSEHOLD_TRAYITEM_TYPE => {
                                    name_size = content[constants::HOUSEHOLD_TRAYITEM_NAME_OFFSET];
                                    offset = constants::HOUSEHOLD_TRAYITEM_NAME_OFFSET;
                                },
                                constants::PLOT_TRAYITEM_TYPE | constants::ROOM_TRAYITEM_TYPE => {
                                    name_size = content[constants::ROOM_PLOT_TRAYITEM_NAME_OFFSET];
                                    offset = constants::ROOM_PLOT_TRAYITEM_NAME_OFFSET;
                                },
                                _ => {
                                    println!("{} Unknown tray item type, skipping file ({}).", "[Warning!]".yellow(), path.display());
                                    return None;
                                }
                            }
                            let name = utils::extract_string(&content, offset + 1, name_size);
                            return Some(TrayItem {
                                path: path.clone(),
                                name,
                                id,
                                file_type,
                                filename: String::from(file_name)
                            });
                        } else {
                            println!("{} Couldn't parse file name, skipping! ({})", "[Warning!]".yellow(), path.display());
                        }
                    } else {
                        println!("{} Couldn't get file name, skipping! ({})", "[Warning!]".yellow(), path.display());
                    }
                },
                Err(_) => {
                    println!("{} Couldn't read tray item file, the whole gallery item will be skipped!", "[Warning!]".yellow());
                }
            }
        }
        return None;
    }
}

pub struct TrayItemFile {
    pub path: PathBuf,
    pub id: u64,
    pub file_type: u32,
    pub filename: String,
}

impl TrayItemFile {
    pub fn new(path: &PathBuf) -> Option<TrayItemFile> {
        if let Some((file_type, id)) = utils::extract_id_and_type(path) {
            if let Some(filename) = path.file_name() {
                if let Some(filename) = filename.to_str() {
                    return Some(TrayItemFile {
                        path: path.clone(),
                        id,
                        file_type,
                        filename: String::from(filename),
                    });
                } else {
                    println!("{} Couldn't parse file name, skipping! ({})", "[Warning!]".yellow(), path.display());
                }
            } else {
                println!("{} Couldn't get file name, skipping! ({})", "[Warning!]".yellow(), path.display());
            }
        } else {
            println!("{} Couldn't extract id and type from file name, skipping! ({})", "[Warning!]".yellow(), path.display());
        }
        return None;
    }
}