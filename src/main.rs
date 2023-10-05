use std::path::PathBuf;
use clap::Parser;

#[macro_use]
extern crate log;

//mod video;
mod image_decode;
mod term;

use crate::{term::{Renderer, RenderSetting}, image_decode::ImgDecoder};

#[derive(Parser, Debug)]
#[command(author, version)]
#[command(about = "A Command-Line Video Player", long_about = None)]
struct Args {
    // #[arg(long, short, value_parser = file_parser)]
    // video_path: PathBuf,

    /// Path where there storages frame images
    #[arg(long, short, value_parser = dir_parser)]
    image_dir_path: PathBuf,

    /// Print Log to Terminal
    #[arg(long, short)]
    logging: bool,

    /// Render Speed
    #[arg(long, default_value_t=30.00)]
    fps: f64,

    #[arg(long)]
    instant_render: bool
}

fn file_parser(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);
    if !path.is_file() {
        Err(format!("Not a vaild file"))
    } else {
        Ok(path)
    }
}

fn dir_parser(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);
    if !path.is_dir() {
        Err(format!("Not a vaild dir"))
    } else {
        Ok(path)
    }
}

fn main() {
    let args = Args::parse();
    // FIXME: ugly
    if args.logging {
        env_logger::Builder::from_env(
            env_logger::Env::default()
            .default_filter_or("info")
        ).init();
    } else {
        env_logger::init();
    }
    
    info!("Creating Renderer");
    let mut renderer = Renderer::new(RenderSetting{ fps: args.fps });
    
    if args.instant_render {
        let mut decoder = ImgDecoder::new(args.image_dir_path);
        
        renderer.init().unwrap();
        for _ in 0..decoder.get_files_count() {
            decoder.play_next_frame(&mut renderer).unwrap();
        }
    } else {
        let data = image_decode::start(args.image_dir_path, &mut renderer).unwrap();
        renderer.init().unwrap();
        renderer.start_render(data).unwrap();
    }
}
