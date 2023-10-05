use anyhow::anyhow;
use crossterm::{
    cursor, terminal, style::{self, Stylize},
    ExecutableCommand, QueueableCommand
};
use image::{Rgb, Pixel};
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

    inited: bool,
    /// Storage last played frame
    last_frame: Vec<Rgb<u16>>,

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
            inited: false,
            last_frame: vec![],
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

    pub fn start_render(&mut self, data: Vec<Vec<Rgb<u16>>>) -> anyhow::Result<()> {
        if !self.inited {
            return Err(anyhow!("Renderer not inited"));
        }
        
        let fps = time::Duration::from_secs_f64(1.0 / self.setting.fps);

        for frame in data {
            let now = time::Instant::now();

            self.render_frame(frame)?;

            if now.elapsed() < fps {
                thread::sleep(fps - now.elapsed());
            }
        }
        Ok(())
    }

    pub fn init(&mut self) -> anyhow::Result<()> {
        self.stdout.execute(cursor::Hide)?
                    .execute(terminal::Clear(terminal::ClearType::All))?;
                    
        self.inited = true;
        Ok(())
    }

    pub fn render_frame(&mut self, data: Vec<Rgb<u16>>) -> anyhow::Result<()> {
        if !self.inited {
            return Err(anyhow!("Renderer not inited"));
        }

        let start_x = (self.terminal_size.0 / 2) - (self.frame_size.0 / 2);

        self.frame_now += 1;

        self.stdout.execute(cursor::MoveTo(0,0))?
        .execute(style::Print(format!("Frame {}", self.frame_now)))?
        .execute(style::Print("\nBy NoFun"))?;

        let mut x = start_x;
        let mut y = 0;

        self.stdout.execute(terminal::BeginSynchronizedUpdate)?;

        for index in 0..data.len() {
            let pixel = data[index];
            let color = pixel.channels();
            let r = color[0] as u8;
            let g = color[1] as u8;
            let b = color[2] as u8;

            if self.last_frame.len() == 0 || self.last_frame[index].channels() != color {
                self.stdout
                .queue(cursor::MoveTo(x, y))?
                .queue(style::PrintStyledContent("█".with(
                    style::Color::Rgb { r, g, b }
                )))?;
            }

            x += 1;
            if x > self.frame_size.0 + start_x - 1 {
                x = start_x;
                y += 1;
                self.stdout.flush()?;
            }
        }

        self.last_frame = data;
        self.stdout.execute(terminal::EndSynchronizedUpdate)?;
        Ok(())
    }
}