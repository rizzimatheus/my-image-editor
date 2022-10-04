mod image_editor;
use eframe::{App, egui, Frame, NativeOptions, run_native};
use eframe::egui::{CentralPanel, Context, vec2};
use crate::egui::{Align, FontId, Layout, SidePanel, TopBottomPanel};
use crate::egui::FontFamily::{Proportional};
use crate::egui::TextStyle::{Body, Button, Monospace};
use crate::image_editor::State;

const PADDIN: f32 = 5.0;
const SIDE_PANEL_SIZE: f32 = 150.0;
const INITIAL_WINDOW_W: f32 = 52.0 * 16.0;
const INITIAL_WINDOW_H: f32 = 52.0 * 9.0;

fn main() {
    let app = image_editor::ImageEditor::new();
    let win_options = NativeOptions {
        icon_data: Some(load_icon("./icon.png")),
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
                ui.add_space(2.0*PADDIN);
                ui.vertical_centered(|ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.add_sized([SIDE_PANEL_SIZE, 25.], egui::Image::new(self.logo().texture_id(ctx), vec2(50.,50.)));

                        ui.add_space(2.0*PADDIN);
                        ui.separator();

                        // Get current context style
                        let mut style = (*ctx.style()).clone();

                        style.text_styles = [
                            (Button, FontId::new(18.0, Proportional))
                        ].into();
                        ui.style_mut().text_styles = style.text_styles;

                        ui.add_space(2.0*PADDIN);
                        // Blur
                        let blur_button = ui.add_sized([120., 40.], egui::Button::new("Blur"));
                        if blur_button.clicked() && self.current_img_path().is_some() && *self.state() != State::Blur {
                            self.set_current_img_edited_path(self.current_img_path().clone());
                            self.set_state(State::Blur);
                            self.clear_effects_values();
                        }
                        ui.add_space(2.0*PADDIN);
                        // Brighten
                        let brighten_button = ui.add_sized([120., 40.], egui::Button::new("Brighten"));
                        if brighten_button.clicked() && self.current_img_path().is_some() {
                            self.set_current_img_edited_path(self.current_img_path().clone());
                            self.set_state(State::Brighten);
                            self.clear_effects_values();
                        }
                        ui.add_space(2.0*PADDIN);
                        // Contrast
                        let contrast_button = ui.add_sized([120., 40.], egui::Button::new("Contrast"));
                        if contrast_button.clicked() && self.current_img_path().is_some() {
                            self.set_current_img_edited_path(self.current_img_path().clone());
                            self.set_state(State::Contrast);
                            self.clear_effects_values();
                        }
                        ui.add_space(2.0*PADDIN);
                        // Flip
                        let flip_button = ui.add_sized([120., 40.], egui::Button::new("Flip Image"));
                        if flip_button.clicked() && self.current_img_path().is_some() {
                            self.set_current_img_edited_path(self.current_img_path().clone());
                            self.set_state(State::Flip);
                            self.clear_effects_values();
                        }
                        ui.add_space(2.0*PADDIN);
                        // Grayscale
                        let grayscale_button = ui.add_sized([120., 40.], egui::Button::new("Grayscale"));
                        if grayscale_button.clicked() && self.current_img_path().is_some() {
                            self.set_current_img_edited_path(self.current_img_path().clone());
                            self.set_state(State::Grayscale);
                            self.clear_effects_values();
                        }
                        ui.add_space(2.0*PADDIN);
                        // Invert
                        let invert_button = ui.add_sized([120., 40.], egui::Button::new("Invert"));
                        if invert_button.clicked() && self.current_img_path().is_some() {
                            self.set_current_img_edited_path(self.current_img_path().clone());
                            self.set_state(State::Invert);
                            self.clear_effects_values();
                        }
                        ui.add_space(2.0*PADDIN);
                        // Rotate
                        let rotate_button = ui.add_sized([120., 40.], egui::Button::new("Rotate"));
                        if rotate_button.clicked() && self.current_img_path().is_some() {
                            self.set_current_img_edited_path(self.current_img_path().clone());
                            self.set_state(State::Rotate);
                            self.clear_effects_values();
                        }
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
                        // Select image button
                        ui.label("Select a image:");
                        let open_file_button = ui.add_sized([60., 20.], egui::Button::new("Open file..."));
                        if open_file_button.clicked() {
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("image", &["png", "jpg", "jpeg"])
                                .pick_file() {
                                    self.set_initial_image_path(Some(path.clone()));
                                    self.set_current_img_path(Some(path.clone()));
                                    self.set_current_img_edited_path(Some(path));
                                    self.reset_version_number();
                            }
                        }
                        if self.initial_image_path().is_some() {
                            ui.monospace(self.initial_image_name().unwrap());
                        }
                    });

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        // Save image button
                        let save_button = ui.add_sized([60., 20.], egui::Button::new("Save"));
                        if save_button.clicked() && self.current_img_path().is_some() {
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("image", &["png", "jpg", "jpeg"])
                                .set_file_name(&*self.initial_image_name().as_ref().unwrap())
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
            .min_height(40.0)
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {

                    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                        if self.current_img_path().is_some() {
                            ui.add_space(2.0*PADDIN);
                            match self.state() {
                                State::Blur => {
                                    let mut scalar = self.intensity();
                                    ui.add(egui::Slider::new(&mut scalar, 0.0..=20.0));
                                    if self.intensity() != scalar {
                                        self.set_intensity(scalar);
                                        self.set_current_img_edited_path(self.apply_blur());
                                    }
                                },
                                State::Contrast => {
                                    let mut scalar = self.intensity();
                                    ui.add(egui::Slider::new(&mut scalar, -20.0..=20.0));
                                    if self.intensity() != scalar {
                                        self.set_intensity(scalar);
                                        self.set_current_img_edited_path(self.apply_contrast());
                                    }
                                },
                                State::Brighten => {
                                    let mut scalar = self.intensity() as i32;
                                    ui.add(egui::Slider::new(&mut scalar, -100..=100));
                                    if self.intensity() != (scalar as f32) {
                                        self.set_intensity(scalar as f32);
                                        self.set_current_img_edited_path(self.apply_brighten());
                                    }
                                },
                                State::Invert => {
                                    let apply_effect_button = ui.add_sized([40., 20.], egui::Button::new("Invert Image"));
                                    if apply_effect_button.clicked() {
                                        self.set_current_img_edited_path(self.apply_invert());
                                    }
                                },
                                State::Grayscale => {
                                    let apply_effect_button = ui.add_sized([40., 20.], egui::Button::new("Convert to Grayscale"));
                                    if apply_effect_button.clicked() {
                                        self.set_current_img_edited_path(self.apply_grayscale());
                                    }
                                },
                                State::Flip => {
                                    let apply_effect_button = ui.add_sized([40., 20.], egui::Button::new("Flip Horizontally"));
                                    if apply_effect_button.clicked() {
                                        self.set_current_img_edited_path(self.apply_flip_horizontal());
                                    }
                                    ui.add_space(PADDIN);
                                    let apply_effect_button = ui.add_sized([40., 20.], egui::Button::new("Flip Vertically"));
                                    if apply_effect_button.clicked() {
                                        self.set_current_img_edited_path(self.apply_flip_vertical());
                                    }
                                    ui.add_space(PADDIN);
                                    let apply_effect_button = ui.add_sized([40., 20.], egui::Button::new("Flip Horizontally + Vertically"));
                                    if apply_effect_button.clicked() {
                                        self.set_current_img_edited_path(self.apply_flip_horizontal_vertical());
                                    }
                                },
                                State::Rotate => {
                                    let apply_effect_button = ui.add_sized([40., 20.], egui::Button::new("Rotate Image 90°"));
                                    if apply_effect_button.clicked() {
                                        self.set_current_img_edited_path(self.apply_rotate90());
                                    }
                                    ui.add_space(PADDIN);
                                    let apply_effect_button = ui.add_sized([40., 20.], egui::Button::new("Rotate Image 180°"));
                                    if apply_effect_button.clicked() {
                                        self.set_current_img_edited_path(self.apply_rotate180());
                                    }
                                    ui.add_space(PADDIN);
                                    let apply_effect_button = ui.add_sized([40., 20.], egui::Button::new("Rotate Image 270°"));
                                    if apply_effect_button.clicked() {
                                        self.set_current_img_edited_path(self.apply_rotate270());
                                    }
                                },
                                State::Waiting => {},
                            }
                        }
                    });

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if *self.state() != State::Waiting {
                            ui.add_space(2.0 * PADDIN);
                            let confirm_button = ui.add_sized([20., 20.], egui::Button::new("Apply Changes"));

                            if confirm_button.clicked() {
                                self.prepare_new_edition();
                                self.set_state(State::Waiting);
                                self.set_current_img_path(self.current_img_edited_path().clone());
                                self.clear_effects_values();
                            }
                        }
                    });
                });
            });



        CentralPanel::default().show(ctx, |ui| {
            // Display Image
            let image_to_display = match *self.state() {
                State::Waiting => self.current_img(),
                _ => self.current_img_edited(),
            };

            if let Some(img) = image_to_display {
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