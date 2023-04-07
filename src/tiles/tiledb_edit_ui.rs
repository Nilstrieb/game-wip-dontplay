use super::TileDb;

pub fn tiledb_edit_ui(ctx: &egui::Context, tile_db: &mut TileDb) {
    egui::Window::new("Tiledb editor").show(ctx, |ui| {
        if ui.button("Add new default").clicked() {
            tile_db.db.push(super::TileDef::default());
        }
    });
}
