use egui_citro3d::Specifics;

fn main() {
    let mut soc = ctru::prelude::Soc::new().expect("Could not obtain Soc");
    let _ = soc.redirect_to_3dslink(true, true);
    egui_citro3d::run_egui(|ctx,specifics| {
        let Specifics {bottom_viewport_id, top_viewport_id, ..} = specifics;
        if ctx.viewport_id() == top_viewport_id {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label("File Contents Go Here");
            });
        }
        if ctx.viewport_id() == bottom_viewport_id {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label("File Tree Goes Here");
            });
        }
    });
}
