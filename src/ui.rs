// Copyright (C) 2026 Xhelgi
// This file is part of Yutikor and is released under the GNU GPL v3.0.

mod content;
mod folder;
mod menu;
mod tools;

use std::fs;

use crate::{
    app::Yuti,
    data::{Node, Page},
};

impl eframe::App for Yuti {
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        if let Some(path) = &self.path {
            menu::save_graph(&self.graph_root_node, path);
        }
    }

    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        if let Some(path) = &self.path.clone() {
            if let Some(crt_page) = &mut self.crt_page {
                // ===== Режим редактирования страницы =====
                tools::create_top_menu(ctx, &mut self.editor_state);
                if self.editor_state.are_tools_visible {
                    tools::create_tools_area(ctx, &mut self.editor_state, crt_page);
                }
                content::create_content_panel(
                    ctx,
                    crt_page,
                    &mut self.graph_state,
                    &mut self.editor_state,
                    path,
                );
                content::logic::sort_by_z(crt_page);
                content::logic::remove_obj_if_need(
                    crt_page,
                    &mut self.editor_state.object_to_remove_id,
                );
                content::logic::hotkey_process(ctx, &mut self.editor_state, crt_page);
            } else if let Some(graph_root) = &mut self.graph_root_node {
                // ===== Режим графа =====
                menu::create_graph_panel(
                    ctx,
                    &mut self.crt_page,
                    graph_root,
                    &mut self.graph_state,
                    &mut self.folder_state,
                    path,
                );
            } else {
                // ===== Загрузка графа с диска =====
                let graph_path = path.join("graph.base");
                if let Ok(json_string) = fs::read_to_string(&graph_path) {
                    match serde_json::from_str(&json_string) {
                        Ok(node) => self.graph_root_node = Some(node),
                        Err(e) => {
                            println!("Error reading GraphFile: {e}");
                            fs::remove_file(&graph_path).expect("Cannot remove broken GraphFile!");
                        }
                    }
                } else {
                    let def_graph = Node::default();
                    let json =
                        serde_json::to_string(&def_graph).expect("Cannot serialize default Node");
                    fs::write(&graph_path, json).expect("Cannot create default GraphFile");
                    self.graph_root_node = Some(def_graph);
                }
            }

            // ===== Команда: закрыть страницу =====
            if self.editor_state.page_to_close {
                if let Some(save_path) = &self.graph_state.node_to_load_by_path.clone()
                    && let Some(crt_page) = &self.crt_page
                {
                    menu::save_page(save_path, path, crt_page);
                }
                self.crt_page = None;
                self.reset_editor_state();
            }

            // ===== Команда: переключить страницу =====
            if let Some(switch_to) = self.graph_state.page_to_switch.clone() {
                if let Some(save_path) = &self.graph_state.node_to_load_by_path.clone()
                    && let Some(crt_page) = &self.crt_page
                {
                    menu::save_page(save_path, path, crt_page);
                }
                self.crt_page = None;
                self.graph_state.page_to_switch = None;
                self.reset_editor_state();

                let full_path = path.join(&switch_to);
                if let Ok(file_string) = fs::read_to_string(&full_path)
                    && let Ok(page) = serde_json::from_str(&file_string)
                {
                    self.crt_page = Some(page);
                } else {
                    let new_page = Page::default();
                    let json = serde_json::to_string_pretty(&new_page)
                        .expect("Cannot serialize default Page");
                    fs::write(&full_path, json).expect("Cannot write default Page file");
                    self.crt_page = Some(new_page);
                }

                self.graph_state.node_to_load_by_path = Some(switch_to.clone());
                if let Some(root) = &self.graph_root_node {
                    self.graph_state.page_links = menu::get_links_by_path(&switch_to, root);
                }
            }
        } else {
            // ===== Нет пути — выбор папки =====
            self.crt_page = None;
            self.graph_root_node = None;
            folder::draw_folder_select_panel(ctx, &mut self.folder_state, &mut self.path);
        }

        // ===== Команда: сбросить путь =====
        if self.folder_state.is_path_to_clear {
            if let Some(path) = &self.path {
                menu::save_graph(&self.graph_root_node, path);
                if let Some(save_path) = &self.graph_state.node_to_load_by_path.clone()
                    && let Some(crt_page) = &self.crt_page
                {
                    menu::save_page(save_path, path, crt_page);
                }
            }
            self.path = None;
            self.crt_page = None;
            self.graph_root_node = None;
            self.folder_state.is_path_to_clear = false;
            self.graph_state.page_links = Vec::new();
            self.graph_state.node_to_load_by_path = None;
        }
    }
}
