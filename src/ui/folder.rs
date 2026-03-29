// Copyright (C) 2026 Xhelgi
// This file is part of Yutikor and is released under the GNU GPL v3.0.

use eframe::egui;
use std::{collections::VecDeque, fs, path::PathBuf};

use crate::app::FolderState;

pub fn draw_folder_select_panel(
    ctx: &egui::Context,
    folder_state: &mut FolderState,
    path: &mut Option<PathBuf>,
) {
    let width = 500.0;
    let height = 40.0;

    let mut last_readed_folders: VecDeque<PathBuf> = VecDeque::new();

    let Some(cache) = dirs::cache_dir() else {
        panic!("Cannot get Cache Dir!")
    };

    let cache_dir = cache.join("yutikor");
    fs::create_dir_all(&cache_dir).expect("Cannot create Cache Dir!");
    let cache_file = cache_dir.join("last_readed");

    if let Ok(json_string) = fs::read_to_string(&cache_file) {
        last_readed_folders = serde_json::from_str(&json_string).unwrap_or_default();
    } else {
        fs::write(&cache_file, "").expect("Cannot write into CacheFile!");
    }

    while last_readed_folders.len() > 10 {
        last_readed_folders.pop_front();
    }

    egui::CentralPanel::default()
        .frame(egui::Frame::NONE.fill(egui::Color32::WHITE))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(200.0);
                ui.add_sized(
                    [width, height],
                    egui::TextEdit::singleline(&mut folder_state.path_line),
                );
                ui.add_space(20.0);

                let mut index_to_remove: Option<usize> = None;
                for (index, f_path) in last_readed_folders.iter().enumerate() {
                    if let Some(text) = f_path.to_str() {
                        let resp = ui.add_sized([width, height], egui::Button::new(text));
                        if resp.clicked() {
                            folder_state.path_line = String::from(text);
                        }
                        resp.context_menu(|ui| {
                            if ui.button("Remove").clicked() {
                                index_to_remove = Some(index);
                            }
                        });
                    } else {
                        index_to_remove = Some(index);
                    }
                }
                if let Some(index) = index_to_remove {
                    last_readed_folders.remove(index);
                }

                ui.add_sized([width, height], |ui: &mut egui::Ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Select").clicked() && !folder_state.path_line.is_empty() {
                            let new_path = PathBuf::from(&folder_state.path_line);
                            let error_id = egui::Id::new("PathErrorPopup");
                            if fs::create_dir_all(&new_path).is_err() {
                                ctx.data_mut(|d| d.insert_temp(error_id, true));
                            } else {
                                *path = Some(new_path.clone());
                                if !last_readed_folders.contains(&new_path) {
                                    last_readed_folders.push_back(new_path);
                                }
                            }
                        }
                    })
                    .response
                });

                // Показываем попап ошибки если нужно
                let error_id = egui::Id::new("PathErrorPopup");
                let err: bool = ctx.data(|d| d.get_temp(error_id).unwrap_or(false));
                if err {
                    egui::Window::new("Error")
                        .collapsible(false)
                        .resizable(false)
                        .show(ctx, |ui| {
                            ui.label("Wrong Folder-Path!");
                            if ui.button("Ok").clicked() {
                                ctx.data_mut(|d| d.insert_temp(error_id, false));
                            }
                        });
                }
            });
        });

    let json_string = serde_json::to_string(&last_readed_folders).unwrap_or_default();
    if let Err(e) = fs::write(&cache_file, json_string) {
        println!("{e}");
    }
}
