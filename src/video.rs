
/*use std::{
    fs::File,
    path::PathBuf,
    ffi::{CStr, CString}
};

use video_rs::{self, Decoder, Locator};

pub fn video_decode(path: PathBuf) -> anyhow::Result<()> {
    video_rs::init()?;
    let source = Locator::Url(path);
    let decoder = Decoder::new(&source)?;

    for frame in decoder.decode_iter() {
        if let Ok((_, frame)) = frame {
            let rgb = frame
                .slice(ndarray::s![0, 0, ..])
                .to_slice()
                .unwrap();
            println!(
                "pixel at 0, 0: {}, {}, {}",
                rgb[0],
                rgb[1],
                rgb[2],
            );
        } else {
            break;
        }
    }

    Ok(())
}*/