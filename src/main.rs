use std::fs;

mod utils;
mod tray_files;
mod constants;

fn main() {
    if let Some(tray_folder) = utils::get_tray_folder() {
        if let Some(output_folder) = utils::prepare_output_folder() {
            let tray_content = utils::read_tray_files(tray_folder);
            for tray_item in tray_content.trayitem_files {
                if let Some(item_type_folder) = utils::preapare_output_folder_for_type(&output_folder, tray_item.file_type) {
                    let hex_id = format!("0x{:x}", tray_item.id);
                    let output_folder_name = format!("{} ({})", tray_item.name, hex_id);
                    let tray_item_folder = item_type_folder.join(output_folder_name);
                    match fs::create_dir(&tray_item_folder) {
                        Ok(_) => {
                            let tray_item_file_path = tray_item_folder.join(&tray_item.filename);
                            match fs::copy(&tray_item.path, tray_item_file_path) {
                                Ok(_) => {
                                    utils::print_success(format!("[{}] {} copied to gallery item folder.", tray_item.name, tray_item.filename));
                                    for unknown_item in &tray_content.unknown_files {
                                        if unknown_item.id == tray_item.id {
                                            let unknown_item_path = tray_item_folder.join(&unknown_item.filename);
                                            match fs::copy(&unknown_item.path, unknown_item_path) {
                                                Ok(_) => {
                                                    utils::print_success(format!("[{}] {} copied to gallery item folder.", tray_item.name, unknown_item.filename));
                                                },
                                                Err(_) => {
                                                    utils::print_error(format!("[{}] Couldn't copy {} to gallery item folder!", tray_item.name, unknown_item.filename));
                                                    utils::print_error(format!("[{}] The resulting gallery item will be corrupted!", tray_item.name));
                                                }
                                            }
                                        }
                                    }
                                    let mut check_sgi = true;
                                    let mut expected_id = tray_item.id + 1;
                                    while check_sgi {
                                        for sgi_file in &tray_content.sgi_files {
                                            if sgi_file.id == expected_id {
                                                let sgi_target_path = tray_item_folder.join(&sgi_file.filename);
                                                match fs::copy(&sgi_file.path, sgi_target_path) {
                                                    Ok(_) => {
                                                        utils::print_success(format!("[{}] {} copied to gallery item folder.", tray_item.name, sgi_file.filename));
                                                    },
                                                    Err(_) => {
                                                        utils::print_error(format!("[{}] Couldn't copy {} to gallery item folder!", tray_item.name, sgi_file.filename));
                                                        utils::print_warning(format!("[{}] The resulting gallery item will be corrupted!", tray_item.name));
                                                    }
                                                }
                                                expected_id += 1;
                                            } else {
                                                check_sgi = false;
                                            }
                                        }
                                    }
                                },
                                Err(_) => {
                                    utils::print_error(format!("[{}] Couldn't copy {} to gallery item folder!", tray_item.name, tray_item.filename));
                                    utils::print_warning(format!("[{}] The whole gallery item will be skipped!", tray_item.name));
                                }
                            }
                        },
                        Err(_) => {
                            utils::print_error(format!("[{}] Couldn't create output folder, the whole item will be skipped!", tray_item.name));
                        }
                    }
                } else {
                    let tray_item_type_name: String;
                    match tray_item.file_type {
                        constants::HOUSEHOLD_TRAYITEM_TYPE => {
                            tray_item_type_name = String::from(constants::HOUSEHOLD_FOLDERNAME);
                        },
                        constants::PLOT_TRAYITEM_TYPE => {
                            tray_item_type_name = String::from(constants::PLOT_FOLDERNAME);
                        },
                        constants::ROOM_TRAYITEM_TYPE => {
                            tray_item_type_name = String::from(constants::ROOM_FOLDERNAME);
                        },
                        _ => {
                            tray_item_type_name = String::from("unknown");
                        }
                    }
                    utils::print_error(format!("Couldn't create output folder for gallery item type {}!", tray_item_type_name));
                    utils::print_warning(format!("All {} gallery items will be skipped!", tray_item_type_name));
                }
            }
        }
    }
    utils::print_info(String::from("Program completed! Press any key to continue..."));
    press_btn_continue::wait("").unwrap();
}