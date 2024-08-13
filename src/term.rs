use anyhow::anyhow;
use crossterm::{
    cursor, terminal, style::{self, Stylize},
    ExecutableCommand, QueueableCommand
};
use image::{ImageBuffer, Pixel, Rgb};
use std::{
    io::{self, Write},
    thread,
    time,
};

fn get_size() -> (u16, u16) {
    // x, y
    return crossterm::terminal::size().unwrap();
}

#[derive(Debug)]
pub struct RenderSetting {
    pub fps: f64,
}

#[derive(Debug)]
pub struct Renderer {
    /// Terminal Size
    terminal_size: (u16, u16),
    /// Frame Size, y should as same as term_size
    frame_size: (u16, u16),

    frame_now: u16,

    stdout: io::Stdout,

    initialized: bool,
    /// Storage last played frame
    last_frame: Option<ImageBuffer<Rgb<u8>, Vec<u8>>>,

    setting: RenderSetting,
}

impl Renderer {
    pub fn new(setting: RenderSetting) -> Self {
        let term_size = get_size();
        info!("Created Renderer");
        info!("Terminal size: {:?}", term_size);
        Self {
            terminal_size: term_size,
            frame_size: term_size,
            frame_now: 0,
            stdout: io::stdout(),
            initialized: false,
            last_frame: None,
            setting,
        }
    }

    /// Set Frame Size (y, x)
    pub fn set_frame_size(&mut self, size: (u16, u16)) {
        self.frame_size = size
    }

    pub fn frame_size(&self) -> (u16, u16) {
        self.frame_size
    }

    pub fn term_size(&self) -> (u16, u16) {
        self.terminal_size
    }

    pub fn start_render(&mut self, data: Vec<ImageBuffer<Rgb<u8>, Vec<u8>>>) -> anyhow::Result<()> {
        if !self.initialized {
            return Err(anyhow!("Renderer not inited"));
        }
        
        let fps = time::Duration::from_secs_f64(1.0 / self.setting.fps);

        // TODO: skip frames
        for frame in data {
            let now = time::Instant::now();

            self.render_frame(frame)?;

            if now.elapsed() < fps {
                thread::sleep(fps - now.elapsed());
            }
        }

        self.stdout.execute(terminal::LeaveAlternateScreen)?;

        Ok(())
    }

    pub fn init(&mut self) -> anyhow::Result<()> {
        self.stdout.execute(terminal::EnterAlternateScreen)?
                    .execute(cursor::Hide)?
                    .execute(terminal::Clear(terminal::ClearType::All))?;
                    
        self.initialized = true;
        Ok(())
    }

    pub fn render_frame(&mut self, data: ImageBuffer<Rgb<u8>, Vec<u8>>) -> anyhow::Result<()> {

        // info!("Frame {}", self.frame_now);

        if !self.initialized {
            return Err(anyhow!("Renderer not inited"));
        }

        //let start_x = (self.terminal_size.0 / 2) - (self.frame_size.0 / 2);

        self.frame_now += 1;

        self.stdout.execute(cursor::MoveTo(0,0))?
            .execute(style::Print(format!("Frame {}", self.frame_now)))?
            .execute(style::Print("\nBy NoFun"))?;

        let mut x = 0;
        let mut y = 0;

        self.stdout.execute(terminal::BeginSynchronizedUpdate)?;

        for row in data.rows() {
            x = 0;

            for pixel in row {
                let colors = pixel.channels();
                let r = colors[0];
                let g = colors[1];
                let b = colors[2];

                self.stdout
                    .queue(cursor::MoveTo(x, y))?
                    .queue(style::PrintStyledContent("█".with(
                        style::Color::Rgb { r, g, b }
                    )))?;
                
                x += 1;
            }

            self.stdout.flush()?;
            y += 1;
        }

        //self.last_frame = Some(data);
        self.stdout.execute(terminal::EndSynchronizedUpdate)?;
        Ok(())
    }
}