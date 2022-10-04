use std::path::{PathBuf};
use std::collections::VecDeque;
use std::fs;
use tempfile::{tempdir, TempDir};
use egui_extras::RetainedImage;

const MAX_VERSIONS: usize = 5;

#[derive(PartialEq)]
pub enum State {
    Waiting,
    Blur,
    Brighten,
    Contrast,
    Flip,
    Grayscale,
    Invert,
    Rotate,
}

pub struct ImageEditor {
    temp_dir: TempDir,
    initial_image_path: Option<PathBuf>,
    current_img: Option<RetainedImage>,
    current_img_path: Option<PathBuf>,
    current_img_edited: Option<RetainedImage>,
    current_img_edited_path: Option<PathBuf>,
    version_number: usize,
    versions: VecDeque<Option<PathBuf>>,
    versions_discart: VecDeque<Option<PathBuf>>,
    logo: RetainedImage,
    state: State,
    intensity: f32,
}

impl ImageEditor {
    pub fn new() -> Self {
        Self {
            temp_dir: tempdir().expect("Não foi possível criar diretório temporário"),
            initial_image_path: None,
            current_img: None,
            current_img_path: None,
            current_img_edited: None,
            current_img_edited_path: None,
            version_number: 0,
            versions: VecDeque::new(),
            versions_discart: VecDeque::new(),
            logo: RetainedImage::from_image_bytes(
                "logo.png",
                include_bytes!("../logo.png"),
            ).unwrap(),
            state: State::Waiting,
            intensity: 0.0,
        }
    }

    pub fn current_img(&self) -> &Option<RetainedImage> {
        &self.current_img
    }
    pub fn current_img_path(&self) -> &Option<PathBuf> {
        &self.current_img_path
    }
    pub fn versions(&self) -> &VecDeque<Option<PathBuf>> {
        &self.versions
    }
    pub fn versions_discart(&self) -> &VecDeque<Option<PathBuf>> {
        &self.versions_discart
    }
    pub fn version_number(&self) -> usize {
        self.version_number
    }

    pub fn set_current_img(&mut self, current_img: Option<RetainedImage>) {
        self.current_img = current_img;
    }
    pub fn set_current_img_path(&mut self, current_img_path: Option<PathBuf>) {
        self.current_img_path = current_img_path.clone();

        let img = fs::read(current_img_path.unwrap()).expect("ERROR READING PICKED IMAGE!");
        self.set_current_img(Some(RetainedImage::from_image_bytes(
            self.initial_image_name().unwrap(),
            &img,
        ).expect("ERROR UPDATING IMAGE FROM PICKED PATH!")));
    }

    pub fn inc_version_number(&mut self) {
        self.version_number += 1;
    }
    pub fn pop_front_versions(&mut self) {
        self.versions.pop_front();
    }
    pub fn push_back_versions(&mut self, new_item: Option<PathBuf>) {
        self.versions.push_back(new_item);
    }
    pub fn pop_back_versions(&mut self) -> Option<PathBuf> {
        self.versions.pop_back().unwrap()
    }
    pub fn push_back_versions_discart(&mut self, new_item: Option<PathBuf>) {
        self.versions_discart.push_back(new_item);
    }
    pub fn pop_back_versions_discart(&mut self) -> Option<PathBuf> {
        self.versions_discart.pop_back().unwrap()
    }
    pub fn clear_versions_discart(&mut self) {
        self.versions_discart.clear();
    }
    pub fn reset_version_number(&mut self) {
        self.version_number = 0;
    }

    fn open_current_image(&self) -> image::DynamicImage {
        image::open(self.current_img_path
            .as_ref()
            .unwrap())
            .expect("Failed to open INFILE.")
    }
    fn get_outfile_pathname(&self) -> PathBuf {
        self.temp_dir.path().join(format!("{}_{}", self.version_number, self.initial_image_name().as_ref().unwrap()))

    }

    pub fn initial_image_name(&self) -> Option<String> {
        Some(String::from(self.initial_image_path.as_ref().unwrap().file_name().unwrap().to_str().unwrap()))
    }
    pub fn prepare_new_edition(&mut self) {
        // VERIFICAR SE CHEGOU NO MÁXIMO DE VERSÕES
        if self.versions().len() > MAX_VERSIONS {
            self.pop_front_versions();
        }
        // Se tiver alguma versão na pilha de descarte, remove
        if !self.versions_discart().is_empty() {
            self.clear_versions_discart();
        }
        // Adiciona na pilha de versões a versão atual
        self.push_back_versions(self.current_img_path().clone());
        // Incrementa o número da versão
        self.inc_version_number();
    }

    pub fn apply_blur(&self) -> Option<PathBuf> {
        if self.intensity == 0.0 {
            return self.current_img_path.clone();
        }

        // Open current image
        let img = self.open_current_image();

        // Apply blur (f32) on current image
        let new_img = img.blur(self.intensity);

        // Save new image on a temp dir
        let file_path = self.get_outfile_pathname();
        println!("FILE_PATH = {:?}", file_path);
        new_img.save(&file_path).expect("Failed writing OUTFILE.");

        // Return new image path
        Some(file_path)
    }
    pub fn apply_brighten(&self) -> Option<PathBuf> {
        // Open current image
        let img = self.open_current_image();
        // Apply brighten (i32) on current image
        let new_img = img.brighten(self.intensity as i32);
        // Save new image on a temp dir
        let file_path = self.get_outfile_pathname();
        new_img.save(&file_path).expect("Failed writing OUTFILE.");
        // Return new image path
        Some(file_path)
    }
    pub fn apply_invert(&self) -> Option<PathBuf> {
        // Open current image
        let mut img = self.open_current_image();
        // Apply brighten (i32) on current image
        img.invert();
        // Save new image on a temp dir
        let file_path = self.get_outfile_pathname();
        img.save(&file_path).expect("Failed writing OUTFILE.");
        // Return new image path
        Some(file_path)
    }
    pub fn apply_grayscale(&self) -> Option<PathBuf> {
        // Open current image
        let img = self.open_current_image();
        // Apply brighten (i32) on current image
        let new_img = img.grayscale();
        // Save new image on a temp dir
        let file_path = self.get_outfile_pathname();
        new_img.save(&file_path).expect("Failed writing OUTFILE.");
        // Return new image path
        Some(file_path)
    }
    pub fn apply_contrast(&self) -> Option<PathBuf> {
        // Open current image
        let img = self.open_current_image();
        // Apply brighten (f32) on current image
        let new_img = img.adjust_contrast(self.intensity);
        // Save new image on a temp dir
        let file_path = self.get_outfile_pathname();
        new_img.save(&file_path).expect("Failed writing OUTFILE.");
        // Return new image path
        Some(file_path)
    }
    pub fn apply_flip_horizontal_vertical(&self) -> Option<PathBuf> {
        // Open current image
        let img = self.open_current_image();
        // Apply brighten (i32) on current image
        let mut new_img = img.fliph();
        new_img = new_img.flipv();
        // Save new image on a temp dir
        let file_path = self.get_outfile_pathname();
        new_img.save(&file_path).expect("Failed writing OUTFILE.");
        // Return new image path
        Some(file_path)
    }
    pub fn apply_flip_horizontal(&self) -> Option<PathBuf> {
        // Open current image
        let img = self.open_current_image();
        // Apply brighten (i32) on current image
        let new_img = img.fliph();
        // Save new image on a temp dir
        let file_path = self.get_outfile_pathname();
        new_img.save(&file_path).expect("Failed writing OUTFILE.");
        // Return new image path
        Some(file_path)
    }
    pub fn apply_flip_vertical(&self) -> Option<PathBuf> {
        // Open current image
        let img = self.open_current_image();
        // Apply brighten (i32) on current image
        let new_img = img.flipv();
        // Save new image on a temp dir
        let file_path = self.get_outfile_pathname();
        new_img.save(&file_path).expect("Failed writing OUTFILE.");
        // Return new image path
        Some(file_path)
    }
    pub fn apply_rotate90(&self) -> Option<PathBuf> {
        // Open current image
        let img = self.open_current_image();
        // Apply brighten (i32) on current image
        let new_img = img.rotate90();
        // Save new image on a temp dir
        let file_path = self.get_outfile_pathname();
        new_img.save(&file_path).expect("Failed writing OUTFILE.");
        // Return new image path
        Some(file_path)
    }
    pub fn apply_rotate180(&self) -> Option<PathBuf> {
        // Open current image
        let img = self.open_current_image();
        // Apply brighten (i32) on current image
        let new_img = img.rotate180();
        // Save new image on a temp dir
        let file_path = self.get_outfile_pathname();
        new_img.save(&file_path).expect("Failed writing OUTFILE.");
        // Return new image path
        Some(file_path)
    }
    pub fn apply_rotate270(&self) -> Option<PathBuf> {
        // Open current image
        let img = self.open_current_image();
        // Apply brighten (i32) on current image
        let new_img = img.rotate270();
        // Save new image on a temp dir
        let file_path = self.get_outfile_pathname();
        new_img.save(&file_path).expect("Failed writing OUTFILE.");
        // Return new image path
        Some(file_path)
    }


    pub fn logo(&self) -> &RetainedImage {
        &self.logo
    }
    pub fn state(&self) -> &State {
        &self.state
    }
    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }
    pub fn current_img_edited(&self) -> &Option<RetainedImage> {
        &self.current_img_edited
    }
    pub fn current_img_edited_path(&self) -> &Option<PathBuf> {
        &self.current_img_edited_path
    }
    pub fn set_current_img_edited(&mut self, current_img_edited: Option<RetainedImage>) {
        self.current_img_edited = current_img_edited;
    }
    pub fn set_current_img_edited_path(&mut self, current_img_edited_path: Option<PathBuf>) {
        self.current_img_edited_path = current_img_edited_path.clone();

        let img = fs::read(current_img_edited_path.unwrap()).expect("ERROR READING PICKED IMAGE!");
        self.set_current_img_edited(Some(RetainedImage::from_image_bytes(
            self.initial_image_name().unwrap(),
            &img,
        ).expect("ERROR UPDATING IMAGE FROM PICKED PATH!")));
    }
    pub fn set_initial_image_path(&mut self, initial_image_path: Option<PathBuf>) {
        self.initial_image_path = initial_image_path;
    }
    pub fn clear_effects_values(&mut self) {
        self.intensity = 0.0;
    }
    pub fn set_intensity(&mut self, intensity: f32) {
        self.intensity = intensity;
    }
    pub fn intensity(&self) -> f32 {
        self.intensity
    }
    pub fn initial_image_path(&self) -> &Option<PathBuf> {
        &self.initial_image_path
    }
}


