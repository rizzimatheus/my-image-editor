mod image_editor;
use std::fs;
use eframe::{App, egui, Frame, NativeOptions, run_native};
use eframe::egui::{CentralPanel, Context, vec2};
use egui_extras::RetainedImage;
use crate::egui::{Align, FontId, Layout, SidePanel, TopBottomPanel};
use crate::egui::FontFamily::{Proportional};
use crate::egui::TextStyle::{Body, Button, Monospace};

const PADDIN: f32 = 5.0;
const SIDE_PANEL_SIZE: f32 = 150.0;
const INITIAL_WINDOW_W: f32 = 52.0 * 16.0;
const INITIAL_WINDOW_H: f32 = 52.0 * 9.0;

fn main() {
    let app = image_editor::ImageEditor::new();
    let win_options = NativeOptions {
        icon_data: Some(load_icon("./rust_logo.png")),
        initial_window_size: Some(vec2(INITIAL_WINDOW_W, INITIAL_WINDOW_H)),
        min_window_size: Some(vec2(0.6*INITIAL_WINDOW_W, 0.6*INITIAL_WINDOW_H)),
        ..Default::default()
    };
    run_native(
        "Image Editor",
        win_options,
        Box::new(|_cc| Box::new(app))
    );
}

impl App for image_editor::ImageEditor {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {

        SidePanel::left("my_left_panel")
            .resizable(false)
            .default_width(SIDE_PANEL_SIZE)
            // .width_range(80.0..=400.0)
            .show(ctx, |ui| {
                ui.add_space(PADDIN);
                ui.vertical_centered(|ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.add_sized([SIDE_PANEL_SIZE, 25.], egui::Image::new(self.logo().texture_id(ctx), vec2(50.,50.)));

                        ui.label(format!("Image Verison = {}", self.version_number()));

                        // Get current context style
                        let mut style = (*ctx.style()).clone();

                        style.text_styles = [
                            (Button, FontId::new(18.0, Proportional))
                        ].into();
                        ui.style_mut().text_styles = style.text_styles;

                        ui.add_space(PADDIN);
                        // Blur
                        let blur_buttom = ui.add_sized([120., 40.], egui::Button::new("Blur"));
                        if blur_buttom.clicked() && self.current_img_path().is_some() {
                            self.prepare_new_edition();
                            self.set_current_img_path(self.blur());
                        }
                        ui.add_space(PADDIN);
                        // Brighten
                        let brighten_buttom = ui.add_sized([120., 40.], egui::Button::new("Brighten"));
                        if brighten_buttom.clicked() && self.current_img_path().is_some() {
                            self.prepare_new_edition();
                            self.set_current_img_path(self.brighten());
                        }
                        ui.add_space(PADDIN);
                        // Contrast
                        let contrast_buttom = ui.add_sized([120., 40.], egui::Button::new("Contrast"));
                        if contrast_buttom.clicked() && self.current_img_path().is_some() {
                            self.prepare_new_edition();
                            self.set_current_img_path(self.contrast());
                        }
                        ui.add_space(PADDIN);
                        // Flip_horizontal
                        let flip_horizontal_buttom = ui.add_sized([120., 40.], egui::Button::new("Flip Horizontal"));
                        if flip_horizontal_buttom.clicked() && self.current_img_path().is_some() {
                            self.prepare_new_edition();
                            self.set_current_img_path(self.flip_horizontal());
                        }
                        ui.add_space(PADDIN);
                        // Flip_vertical
                        let flip_vertical_buttom = ui.add_sized([120., 40.], egui::Button::new("Flip Vertical"));
                        if flip_vertical_buttom.clicked() && self.current_img_path().is_some() {
                            self.prepare_new_edition();
                            self.set_current_img_path(self.flip_vertical());
                        }
                        ui.add_space(PADDIN);
                        // Grayscale
                        let grayscale_buttom = ui.add_sized([120., 40.], egui::Button::new("Grayscale"));
                        if grayscale_buttom.clicked() && self.current_img_path().is_some() {
                            self.prepare_new_edition();
                            self.set_current_img_path(self.grayscale());
                        }
                        ui.add_space(PADDIN);
                        // Invert
                        let invert_buttom = ui.add_sized([120., 40.], egui::Button::new("Invert"));
                        if invert_buttom.clicked() && self.current_img_path().is_some() {
                            self.prepare_new_edition();
                            self.set_current_img_path(self.invert());
                        }
                        ui.add_space(PADDIN);
                        // Rotate90
                        let rotate90_buttom = ui.add_sized([120., 40.], egui::Button::new("Rotate 90°"));
                        if rotate90_buttom.clicked() && self.current_img_path().is_some() {
                            self.prepare_new_edition();
                            self.set_current_img_path(self.rotate90());
                        }
                        ui.add_space(PADDIN);

                        // Teste
                        // #[derive(PartialEq)]
                        // enum State { Waiting, Blur, Brighten, Contrast, FlipH, FlipV, Grayscale, Invert, Rotate90 }
                        // let mut my_state = State::Waiting;

                        ui.add_space(PADDIN);

                    });
                });
            });


        TopBottomPanel::top("top_panel")
            .resizable(false)
            // .default_height(10.)
            // .min_height(50.0)
            .show(ctx, |ui| {
                ui.add_space(PADDIN);
                ui.horizontal(|ui| {

                    ui.add_space(PADDIN);

                    // Get current context style
                    let mut style = (*ctx.style()).clone();

                    style.text_styles = [
                        (Button, FontId::new(16.0, Proportional)),
                        (Body, FontId::new(16.0, Proportional)),
                        (Monospace, FontId::new(14.0, Proportional)),
                    ].into();
                    ui.style_mut().text_styles = style.text_styles;

                    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                        ui.label("Select a image:");
                        let open_file_button = ui.add_sized([60., 20.], egui::Button::new("Open file..."));
                        if open_file_button.clicked() {
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("image", &["png", "jpg", "jpeg"])
                                .pick_file() {
                                    self.set_current_img_path(Some(path.clone()));
                                    self.set_image_name(Some(String::from(path.file_name().unwrap().to_str().unwrap())));
                                    self.reset_version_number();
                            }
                        }

                        if let Some(picked_path) = &self.current_img_path() {
                            let file_name = picked_path.file_name().unwrap().to_str().unwrap();
                            ui.monospace(file_name);

                            let img = fs::read(picked_path).expect("ERROR READING PICKED IMAGE!");
                            self.set_current_img(Some(RetainedImage::from_image_bytes(
                                file_name,
                                &img,
                            ).expect("ERROR UPDATING IMAGE FROM PICKED PATH!")));
                        }

                    });

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        let save_button = ui.add_sized([60., 20.], egui::Button::new("Save"));
                        if save_button.clicked() && self.current_img_path().is_some() {
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("image", &["png", "jpg", "jpeg"])
                                .set_file_name(&*self.image_name().as_ref().unwrap())
                                .save_file() {
                                let img = image::open(self.current_img_path().as_ref().unwrap()).expect("Failed to open INFILE.");
                                img.save(path).expect("Failed writing OUTFILE.");
                            }
                        }
                        let redo_button = ui.add_sized([20., 20.], egui::Button::new("▶"));
                        if redo_button.clicked() && !self.versions_discart().is_empty() {
                            // current_img_path > versions
                            self.push_back_versions(self.current_img_path().clone());

                            // versions_discart.pop() > current_img_path
                            let new_current_img_path = self.pop_back_versions_discart();
                            self.set_current_img_path(new_current_img_path);
                        }

                        let undo_button = ui.add_sized([20., 20.], egui::Button::new("◀"));
                        if undo_button.clicked() && !self.versions().is_empty() {
                            // current_img_path > versions_discart
                            self.push_back_versions_discart(self.current_img_path().clone());

                            // versions.pop() > current_img_path
                            let new_current_img_path = self.pop_back_versions();
                            self.set_current_img_path(new_current_img_path);
                        }
                    })
                });
                ui.add_space(PADDIN);
            });


        TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .min_height(50.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Bottom Panel");
                });
            });



        CentralPanel::default().show(ctx, |ui| {
            // Display Image
            if let Some(img) = &self.current_img() {
                let max_size = ui.available_size();
                img.show_max_size(ui, max_size);
            }
        });
    }
}

fn load_icon(path: &str) -> eframe::IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    eframe::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}