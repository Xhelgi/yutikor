// Copyright (C) 2026 Xhelgi
// This file is part of Yutikor and is released under the GNU GPL v3.0.

use eframe::egui;

use crate::{app::EditorState, data::Page};

pub fn sort_by_z(crt_page: &mut Page) {
    crt_page.objects.sort_by_key(|a| a.z_index);
}

pub fn remove_obj_if_need(crt_page: &mut Page, id_to_remove: &mut Option<usize>) {
    if let Some(id) = id_to_remove {
        crt_page.objects.remove(*id);
        *id_to_remove = None;
    }
}

pub fn hotkey_process(ctx: &egui::Context, editor_state: &mut EditorState, crt_page: &mut Page) {
    let (copy, paste, delete, escape) = ctx.input(|i| {
        (
            i.key_pressed(egui::Key::C),
            i.key_pressed(egui::Key::V),
            i.key_pressed(egui::Key::Delete),
            i.key_pressed(egui::Key::Escape),
        )
    });
    let mouse_pos = ctx
        .input(|i| i.pointer.interact_pos())
        .unwrap_or(egui::Pos2::ZERO);

    copy_paste_process(copy, paste, editor_state, crt_page, mouse_pos);
    delete_process(delete, editor_state, crt_page);
    close_process(escape, editor_state);
}

fn copy_paste_process(
    copy: bool,
    paste: bool,
    editor_state: &mut EditorState,
    crt_page: &mut Page,
    mouse_pos: egui::Pos2,
) {
    if copy
        && !editor_state.is_selected_for_text_edit
        && let Some(index) = editor_state.selected_object_id
        && let Some(object) = crt_page.objects.get(index)
    {
        editor_state.copied_object = Some(object.clone());
    }
    if paste
        && !editor_state.is_selected_for_text_edit
        && let Some(item) = editor_state.copied_object.as_ref()
    {
        let mut object = item.clone();
        object.pos.0 = mouse_pos.x;
        object.pos.1 = mouse_pos.y;
        crt_page.objects.push(object);
    }
}

fn delete_process(delete: bool, editor_state: &mut EditorState, crt_page: &mut Page) {
    if delete
        && !editor_state.is_selected_for_text_edit
        && let Some(index) = editor_state.selected_object_id
    {
        crt_page.objects.remove(index);
        editor_state.selected_object_id = None;
    }
}

fn close_process(close: bool, editor_state: &mut EditorState) {
    if close {
        editor_state.page_to_close = true;
    }
}
