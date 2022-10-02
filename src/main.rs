use std::collections::VecDeque;
use std::fs;
use std::path::{PathBuf};
use eframe::{App, egui, Frame, NativeOptions, run_native};
use eframe::egui::{CentralPanel, Context, vec2};
use egui_extras::RetainedImage;
use tempfile::{tempdir, TempDir};
use crate::egui::{Align, Layout, SidePanel};

const PADDIN: f32 = 5.0;
const MAX_VERSIONS: usize = 5;

fn main() {
    let app = ImageEditor {
        temp_dir: tempdir().expect("Não foi possível criar diretório temporário"),
        image_name: None,
        current_img: None,
        current_img_path: None,
        version_number: 1,
        versions: VecDeque::new(),
        versions_discart: VecDeque::new(),
    };
    let win_options = NativeOptions {
        initial_window_size: Some(vec2(800.0, 450.0)),
        min_window_size: Some(vec2(800.0, 450.0)),
        ..Default::default()
    };
    run_native(
        "Image Editor",
        win_options,
        Box::new(|_cc| Box::new(app))
    );
}

struct ImageEditor {
    temp_dir: TempDir,
    image_name: Option<String>,
    current_img: Option<RetainedImage>,
    current_img_path: Option<PathBuf>,
    version_number: usize,
    versions: VecDeque<Option<PathBuf>>,
    versions_discart: VecDeque<Option<PathBuf>>,
}

// impl ImageEditor {
//     fn update_current_image(&mut self, img_path: &PathBuf) {
//         let img = fs::read(img_path).expect("ERROR READING PICKED IMAGE!");
//         self.current_img = Some(RetainedImage::from_image_bytes(
//             img_path.file_name().unwrap().to_str().unwrap(),
//             &img,
//         ).expect("ERROR UPDATING IMAGE FROM PICKED PATH!"));
//     }
// }

impl App for ImageEditor {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        SidePanel::left("my_left_panel")
            .resizable(false)
            .default_width(200.0)
            // .width_range(80.0..=200.0)
            .show(ctx, |ui| {
                ui.add_space(PADDIN);
                ui.vertical_centered(|ui| {
                    ui.heading("My Image Editor");

                    ui.separator();

                    ui.label(format!("Image Verison = {}", self.version_number));

                    if ui.button("Blur").clicked() && self.current_img.is_some() {
                        // VERIFICAR SE CHEGOU NO MÁXIMO DE VERSÕES
                        if self.versions.len() > MAX_VERSIONS {
                            self.versions.pop_front();
                        }
                        // Se tiver alguma versão na pilha de descarte
                        if self.versions_discart.len() > 0 {
                            self.versions_discart.clear();
                        }
                        self.versions.push_back(self.current_img_path.clone());
                        self.current_img_path = Some(blur(self.current_img_path.as_ref().unwrap(), format!("{}_{}", self.version_number, self.image_name.as_ref().unwrap()), &self.temp_dir));
                        self.version_number += 1;
                    }

                });


            });

        CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    ui.label("Select a image:");
                    if ui.button("Open file…").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("image", &["png", "jpg", "jpeg"])
                            .pick_file() {
                                self.current_img_path = Some(path.clone());
                                self.image_name = Some(String::from(path.file_name().unwrap().to_str().unwrap()));
                        }
                    }

                    if let Some(picked_path) = &self.current_img_path {
                        let file_name = picked_path.file_name().unwrap().to_str().unwrap();
                        ui.monospace(file_name);

                        let img = fs::read(picked_path).expect("ERROR READING PICKED IMAGE!");
                        self.current_img = Some(RetainedImage::from_image_bytes(
                            file_name,
                            &img,
                        ).expect("ERROR UPDATING IMAGE FROM PICKED PATH!"));
                    }

                });

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui.button("Save").clicked() && self.current_img_path.is_some() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("image", &["png", "jpg", "jpeg"])
                            .set_file_name(&*self.image_name.as_ref().unwrap())
                            .save_file() {
                                let img = image::open(self.current_img_path.as_ref().unwrap()).expect("Failed to open INFILE.");
                                img.save(path).expect("Failed writing OUTFILE.");
                        }
                    }
                    if ui.button("▶").clicked() && !self.versions_discart.is_empty() {
                        // current_img_path > versions
                        self.versions.push_back(self.current_img_path.clone());
                        // versions_discart.pop() > current_img_path
                        self.current_img_path = self.versions_discart.pop_back().unwrap();
                    }
                    if ui.button("◀").clicked() && !self.versions.is_empty() {
                        // current_img_path > versions_discart
                        self.versions_discart.push_back(self.current_img_path.clone());
                        // versions.pop() > current_img_path
                        self.current_img_path = self.versions.pop_back().unwrap();
                    }
                })
            });

            ui.separator();

            // Display Image
            if let Some(img) = &self.current_img {
                let max_size = ui.available_size();
                img.show_max_size(ui, max_size);
            }
        });
    }
}

fn blur(infile: &PathBuf, outfile: String, dir: &TempDir) -> PathBuf {
    // Open current image
    let img = image::open(infile).expect("Failed to open INFILE.");

    // Apply blur (f32) on current image
    let img2 = img.blur(2.0);

    // Save new image on a temp dir
    let file_path = dir.path().join(outfile);
    println!("FILE_PATH = {:?}", file_path);
    img2.save(&file_path).expect("Failed writing OUTFILE.");

    // Return new image path
    file_path
}