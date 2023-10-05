use anyhow::anyhow;
use image::{self, imageops::FilterType, Rgb};
use indicatif::ProgressBar;
use std::{fs, path::PathBuf};

use crate::term::Renderer;

pub fn start(dir_path: PathBuf, renderer: &mut Renderer) -> anyhow::Result<Vec<Vec<Rgb<u16>>>> {
    //let terminal_w: u16;
    
    let files_count = fs::read_dir(&dir_path).unwrap().count();
    info!("Found {} files in {}", files_count, dir_path.display());
    
    let pb = ProgressBar::new(files_count.try_into()?);
    let mut frames: Vec<_> = vec![];
    
    let mut term_renderer = renderer;

    info!("Decoding Frames");
    for i in fs::read_dir(&dir_path).unwrap() {
        let path =i.unwrap().path();

        //info!("Processing {}", path.display());
        //pb.println(format!("Processing {}", path.display()));

        //p = buffer.pixels_mut();
        
        frames.push(load_img(path, &mut term_renderer)?);
        pb.inc(1);
    }

    /*for i in frames {
        for j in i {
            println!("{:?}", j.channels()[0] as u8);
        }
    }*/

    Ok(frames)
}

fn load_img(path: PathBuf, renderer: &mut Renderer) -> anyhow::Result<Vec<Rgb<u16>>>{
    let mut image = image::open(path)?;
    let terminal_size = renderer.term_size();

    //let image_w = image.width();
    //let image_h = image.height();

    /*image = image.resize_exact(
        <u16 as Into<u32>>::into(terminal_size.0) * 2, 
        terminal_size.1.into(), 
        FilterType::Nearest
    );*/
    image = image.resize(
        terminal_size.0.into(),
        terminal_size.1.into(), 
        FilterType::Nearest
    );

    image = image.resize_exact(
        image.width() * 2, 
        image.height(), 
        FilterType::Nearest
    );

    //println!("{}x{} -> {}x{}", image_w, image_h, image.width(), image.height());

    //let buffer = image.to_rgba16();
    let buffer = image.to_rgb16();
    let mut pixels: Vec<Rgb<u16>> = vec![];
    for y in 0..image.height() {
        for x in 0..image.width() {
        pixels.push(*buffer.get_pixel(x, y));
        }
    }

    let f_size = (image.width() as u16, image.height() as u16);
    if renderer.frame_size() != f_size {
        renderer.set_frame_size(f_size);
    }

    Ok(pixels)
}

pub struct ImgDecoder {
    dir_path: PathBuf,
    dir_reader: fs::ReadDir,
}

impl ImgDecoder {
    pub fn new(dir_path: PathBuf) -> Self {
        Self {
            dir_path: dir_path.clone(),
            dir_reader: fs::read_dir(dir_path).unwrap(),
        }
    }

    pub fn play_next_frame(&mut self, renderer: &mut Renderer) -> anyhow::Result<()> {
        let f = self.dir_reader.next();
        if f.is_none() {
            return Err(anyhow!("End of dir"))
        }
        let f = f.unwrap()?;
        let data = load_img(f.path(), renderer)?;

        renderer.render_frame(data)?;

        Ok(())
    }

    pub fn get_files_count(&self) -> usize {
        fs::read_dir(&self.dir_path).unwrap().count()
    }
}