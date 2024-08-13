use anyhow::anyhow;
use image::{self, ImageBuffer, Rgb};
use indicatif::ProgressBar;
use std::{fs, path::PathBuf};

use fast_image_resize::{CpuExtensions, IntoImageView, ResizeAlg, ResizeOptions, Resizer};
use fast_image_resize::images::Image;

use crate::term::Renderer;

pub fn full_load(dir_path: PathBuf, renderer: &mut Renderer) -> anyhow::Result<Vec<ImageBuffer<Rgb<u8>, Vec<u8>>>> {   
    let files_count = fs::read_dir(&dir_path).unwrap().count();
    info!("Found {} files in {}", files_count, dir_path.display());
    
    // Create progress bar
    let pb = ProgressBar::new(files_count.try_into()?);
    
    let mut frames: Vec<_> = vec![];
    let mut term_renderer = renderer;

    info!("Decoding Frames");

    // Load frames from directory, using the default sort by name.
    for i in fs::read_dir(&dir_path).unwrap() {
        let path =i.unwrap().path();

        frames.push(load_img(path, &mut term_renderer)?);
        pb.inc(1);
    }

    Ok(frames)
}

pub fn check_cpu_extensions() {
    info!("AVX2 support: {}", CpuExtensions::Avx2.is_supported());
    info!("SSE4.1 support: {}", CpuExtensions::Sse4_1.is_supported());
}

fn load_img(path: PathBuf, renderer: &mut Renderer) -> anyhow::Result<ImageBuffer<Rgb<u8>, Vec<u8>>>{
    // load from disk
    let image = image::open(path)?;
    
    // resize image
    let w_scale = image.width() as f32 / image.height() as f32; // float is needed

    let terminal_size = renderer.term_size();
    let w = (terminal_size.1 as f32 * w_scale).ceil() as u16;
    let h = terminal_size.1;

    let mut dst_image = Image::new(
        (w * 2).into(), // must, because console font is 1x2
        h.into(),
        image.pixel_type().unwrap(),
    );

    let mut resizer = Resizer::new();
    let options = ResizeOptions::new().resize_alg(ResizeAlg::Nearest); // Use `Nearest` to speedup processing
    resizer.resize(&image, &mut dst_image, &options).unwrap();

    // save into buffer

    let raw_buffer = dst_image.buffer().to_owned();

    let buffer = ImageBuffer::from_raw(
        dst_image.width(),
        dst_image.height(),
        raw_buffer,
    ).unwrap();

    /*let f_size = (dst_image.width() as u16, dst_image.height() as u16);
    if renderer.frame_size() != f_size {
        renderer.set_frame_size(f_size);
    }*/

    Ok(buffer)
}

pub struct InstantImageDecoder {
    dir_path: PathBuf,
    dir_reader: fs::ReadDir,
}

impl InstantImageDecoder {
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