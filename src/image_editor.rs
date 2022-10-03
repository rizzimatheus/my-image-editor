use std::path::{PathBuf};
use std::collections::VecDeque;
use tempfile::{tempdir, TempDir};
use egui_extras::RetainedImage;
use image::imageops::colorops;

const MAX_VERSIONS: usize = 5;

pub struct ImageEditor {
    temp_dir: TempDir,
    image_name: Option<String>,
    current_img: Option<RetainedImage>,
    current_img_path: Option<PathBuf>,
    version_number: usize,
    versions: VecDeque<Option<PathBuf>>,
    versions_discart: VecDeque<Option<PathBuf>>,
    logo: RetainedImage,
}

impl ImageEditor {
    pub fn new() -> Self {
        Self {
            temp_dir: tempdir().expect("Não foi possível criar diretório temporário"),
            image_name: None,
            current_img: None,
            current_img_path: None,
            version_number: 0,
            versions: VecDeque::new(),
            versions_discart: VecDeque::new(),
            logo: RetainedImage::from_image_bytes(
                "logo.png",
                include_bytes!("../logo.png"),
            ).unwrap(),
        }

    }

    // fn update_current_image(&mut self, img_path: &PathBuf) {
    //     let img = fs::read(img_path).expect("ERROR READING PICKED IMAGE!");
    //     self.current_img = Some(RetainedImage::from_image_bytes(
    //         img_path.file_name().unwrap().to_str().unwrap(),
    //         &img,
    //     ).expect("ERROR UPDATING IMAGE FROM PICKED PATH!"));
    // }


    pub fn temp_dir(&self) -> &TempDir {
        &self.temp_dir
    }
    pub fn image_name(&self) -> &Option<String> {
        &self.image_name
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

    pub fn set_image_name(&mut self, image_name: Option<String>) {
        self.image_name = image_name;
    }
    pub fn set_current_img(&mut self, current_img: Option<RetainedImage>) {
        self.current_img = current_img;
    }
    pub fn set_current_img_path(&mut self, current_img_path: Option<PathBuf>) { // *******************
        self.current_img_path = current_img_path;
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
    pub fn pop_back_versions(&mut self) -> Option<PathBuf> { // *******************
        self.versions.pop_back().unwrap()
    }
    pub fn push_back_versions_discart(&mut self, new_item: Option<PathBuf>) {
        self.versions_discart.push_back(new_item);
    }
    pub fn pop_back_versions_discart(&mut self) -> Option<PathBuf> { // *******************
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
        self.temp_dir.path().join(format!("{}_{}", self.version_number, self.image_name.as_ref().unwrap()))

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

    pub fn blur(&self) -> Option<PathBuf> {
        // Open current image
        let img = self.open_current_image();

        // Apply blur (f32) on current image
        let new_img = img.blur(2.0);

        // Save new image on a temp dir
        let file_path = self.get_outfile_pathname();
        println!("FILE_PATH = {:?}", file_path);
        new_img.save(&file_path).expect("Failed writing OUTFILE.");

        // Return new image path
        Some(file_path)
    }
    pub fn brighten(&self) -> Option<PathBuf> {
        // Open current image
        let img = self.open_current_image();
        // Apply brighten (i32) on current image
        let new_img = img.brighten(-100);
        // Save new image on a temp dir
        let file_path = self.get_outfile_pathname();
        new_img.save(&file_path).expect("Failed writing OUTFILE.");
        // Return new image path
        Some(file_path)
    }
    pub fn invert(&self) -> Option<PathBuf> {
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
    pub fn grayscale(&self) -> Option<PathBuf> {
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
    pub fn contrast(&self) -> Option<PathBuf> {
        // Open current image
        let img = self.open_current_image();
        // Apply brighten (f32) on current image
        let new_img = img.adjust_contrast(20.);
        // Save new image on a temp dir
        let file_path = self.get_outfile_pathname();
        new_img.save(&file_path).expect("Failed writing OUTFILE.");
        // Return new image path
        Some(file_path)
    }
    pub fn flip_horizontal(&self) -> Option<PathBuf> {
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
    pub fn flip_vertical(&self) -> Option<PathBuf> {
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
    pub fn rotate90(&self) -> Option<PathBuf> {
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


    pub fn logo(&self) -> &RetainedImage {
        &self.logo
    }
}


