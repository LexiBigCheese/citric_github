#![feature(result_flattening)]

pub mod repotree;

use egui_citro3d::Specifics;
use repotree::{Fetchable, contents::INode};

fn main() {
    let mut soc = ctru::prelude::Soc::new().expect("Could not obtain Soc");
    let _ = soc.redirect_to_3dslink(true, true);
    let mut fetchable: Fetchable<INode> = Fetchable::new(ehttp::Request::get(
        "https://api.github.com/repos/LexiBigCheese/citric_github/contents",
    ))
    .with_headers();
    let mut selected = 0usize;
    egui_citro3d::run_egui(|ctx, specifics| {
        let Specifics {
            bottom_viewport_id,
            top_viewport_id,
            hid,
        } = specifics;
        if ctx.viewport_id() == top_viewport_id {
            egui::CentralPanel::default().show(ctx, |ui| {
                let mut next_fetchable = None;
                {
                    let resp = fetchable.response.lock().unwrap();
                    let Some(resp) = resp.as_ref() else {
                        return;
                    };
                    match resp {
                        Ok(inode) => {
                            next_fetchable =
                                repotree::contents::view::view_big(ui, hid, inode, &mut selected);
                        }
                        Err(msg) => {
                            ui.colored_label(egui::Color32::RED, format!("ERROR: {}", msg));
                        }
                    };
                }
                if let Some(f) = next_fetchable {
                    fetchable = f;
                }
            });
        }
        if ctx.viewport_id() == bottom_viewport_id {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.text_edit_singleline(&mut fetchable.request.url);
                if ui.button("Execute").clicked() {
                    fetchable.perform();
                }
            });
        }
    });
}
