use std::path::PathBuf;
use clap::Parser;

#[macro_use]
extern crate log;

//mod video;
mod image_decode;
mod term;

use crate::{term::{Renderer, RenderSetting}, image_decode::InstantImageDecoder};

#[derive(Parser, Debug)]
#[command(author, version)]
#[command(about = "A Command-Line Video Player", long_about = None)]
struct Args {
    // #[arg(long, short, value_parser = file_parser)]
    // video_path: Option<PathBuf>,

    /// Path where there storages frame images
    #[arg(long, short, value_parser = dir_parser)]
    image_dir_path: PathBuf,

    /// Print Log to Terminal
    #[arg(long, short)]
    verbose: bool,

    /// Render Speed
    #[arg(long, default_value_t=30.00)]
    fps: f64,

    /// Render Frames while Reading
    #[arg(long)]
    instant_render: bool
}

/*
fn file_parser(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);
    if !path.is_file() {
        Err(format!("Not a vaild file"))
    } else {
        Ok(path)
    }
}
*/

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
    if args.verbose {
        env_logger::Builder::from_env(
            env_logger::Env::default()
            .default_filter_or("info")
        ).init();
    } else {
        env_logger::init();
    }
    
    info!("Creating Renderer");
    let mut renderer = Renderer::new(RenderSetting{
        fps: args.fps,
    });
    
    image_decode::check_cpu_extensions();

    if args.instant_render {
        let mut decoder = InstantImageDecoder::new(args.image_dir_path);
        
        renderer.init().unwrap();
        for _ in 0..decoder.get_files_count() {
            decoder.play_next_frame(&mut renderer).unwrap();
        }
    } else {
        let data = image_decode::full_load(args.image_dir_path, &mut renderer).unwrap();
        renderer.init().unwrap();
        renderer.start_render(data).unwrap();
    }
}
