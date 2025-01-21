use ctru::prelude::{Hid, KeyPad};

use crate::repotree::Fetchable;

use super::INode;
use super::INodeContent::{File,Dir};

pub fn view_big(ui: &mut egui::Ui, hid: &Hid, inode: &INode, selected: &mut usize) -> Option<Fetchable<INode>> {
    let mut next_fetchable = None;
    match &inode.content {
        File { .. } => {
            ui.label(format!("FILE: {}",inode.name));
        },
        Dir { entries } => {
            ui.label(format!("DIR: {}",inode.name));
            let Some(entries) = entries else {
                return None;
            };
            if hid.keys_down().contains(KeyPad::DPAD_DOWN) {
                *selected += 1;
            }
            if hid.keys_down().contains(KeyPad::DPAD_UP) {
                *selected = selected.saturating_sub(1);
            }
            *selected = (*selected).min(entries.len() - 1);
            let row_height = ui.text_style_height(&egui::TextStyle::Body);
            egui::ScrollArea::vertical().show_rows(ui,row_height,entries.len(),|ui,range| {
                for i in range {
                    let this_row_selected = i==*selected;
                    let entry = &entries[i];
                    view_row(ui,entry,this_row_selected);
                    if this_row_selected && hid.keys_down().contains(KeyPad::A) {
                        let f = Fetchable::new(ehttp::Request::get(entry.url.clone())).with_headers();
                        f.perform();
                        next_fetchable = Some(f);
                    }
                }
            });
        },
    }
    next_fetchable
}

pub fn view_row(ui: &mut egui::Ui, inode: &INode, selected: bool) {
    ui.horizontal(|ui| {
        if selected {
            ui.label("> ");
        }
        match inode.content {
            File { .. } => ui.label("FILE: "),
            Dir { .. } => ui.label("DIR: "),
        };
        ui.label(format!("{}",inode.name));
    });
}
