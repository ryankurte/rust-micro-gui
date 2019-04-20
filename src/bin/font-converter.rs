use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::Command;

extern crate sdl2;

use std::env;
use std::path::Path;

use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use sdl2::surface::Surface;

extern crate structopt;
use structopt::StructOpt;

#[macro_use]
extern crate quote;

extern crate proc_macro2;
use proc_macro2::TokenStream;

#[derive(Debug, Clone, StructOpt)]
pub struct Config {
    #[structopt(
        short = "f",
        long = "font",
        default_value = "./fonts/SourceCodePro-Regular.ttf"
    )]
    /// Font to be rendered
    pub font: String,

    #[structopt(short = "s", long = "size", default_value = "12")]
    /// Font render size
    pub size: u16,

    #[structopt(short = "o", long = "output", default_value = "./output.rs")]
    /// Output font file
    pub output: String,

    #[structopt(long = "start-char", default_value = "32")]
    /// Ascii start character code
    pub start: u8,
    #[structopt(long = "end-char", default_value = "33")]
    /// Ascii end character code
    pub end: u8,
}

struct Font {
    pub start: u8,
    pub end: u8,
    pub size: u8,
    pub height: u8,
    pub chars: Vec<Char>,
}

struct Char {
    pub width: u8,
    pub data: Vec<u8>,
}

fn main() -> Result<(), String> {
    let config = Config::from_args();

    let font_name = PathBuf::from(&config.font)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let font = ttf_context.load_font(&config.font, config.size)?;

    let mut surfaces = Vec::with_capacity((config.end - config.start) as usize);

    let mut max_width = 0;
    let mut max_height = 0;

    // Create surfaces for each character
    for i in config.start..config.end {
        let c = i as char;

        // Render character into partial surface
        let s = font
            .render_char(c)
            .solid(Color::RGBA(255, 0, 255, 255))
            .map_err(|e| e.to_string())?;

        if s.width() > max_width {
            max_width = s.width();
        }

        if s.height() > max_height {
            max_height = s.height();
        }

        surfaces.push(s);
    }

    // Generate bitmaps from surfaces
    let padded_width = if max_width % 8 != 0 {
        max_width / 8 + 1
    } else {
        max_width / 8
    } as usize;

    let char_size = padded_width * max_height as usize;

    let mut chars = Vec::with_capacity((config.end - config.start) as usize);
    
    
    let surface = Surface::new(max_width, max_height, PixelFormatEnum::Index8).unwrap();
    let mut canvas = surface.into_canvas().unwrap();
    let texture_creator = canvas.texture_creator();

    // Render out these surfaces
    for s in surfaces {

        // Generate texture from surface
        let texture = texture_creator.create_texture_from_surface(&s)
        .map_err(|e| e.to_string())?;

        // Render into canvas
        canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
        canvas.clear();
        canvas.copy(&texture, None, None)?;
        canvas.present();

        // For some reason render_char does not appear to do anything...
        let pixels = canvas.read_pixels(s.rect(), s.pixel_format_enum()).unwrap();
        println!("px: {:?}", pixels);

        // Always zeros..?!@
        let s = canvas.surface();
        s.with_lock(|pixels| println!("pk: {:?}", pixels) );


        // Convert into char object
        let mut data = vec![0u8; char_size as usize];
        let d = &mut data[..];

        let pixel_size = s.pixel_format_enum().byte_size_per_pixel() as usize;
        let width = s.width() as usize;
        let height = s.height() as usize;

        s.with_lock(|pixels| {
            for y in 0..height {
                for x in 0..width {
                    let p = pixels[(y * width + x) as usize * pixel_size];
                    let b = &mut d[y * padded_width + x / 8];

                    if p != 0 {
                        *b |= 1 << (x % 8);
                    }
                }
            }
        });

        let c = Char {
            width: width as u8,
            data,
        };

        chars.push(c);
    }

    let name = format!("{}_{}pt", font_name, config.size);
    let start = config.start;
    let end = config.end;
    let size = config.size as u8;
    let height = max_height as u8;

    let tokens: Vec<_> = chars
        .iter()
        .map(|c| {
            let width = c.width;
            let data = &c.data;

            quote! {
                Char{
                    width: #width,
                    data: &[#(#data),*],
                }
            }
        })
        .collect();

    let font_map = quote! {
        extern crate micro_gui;
        use micro_gui::prelude::*;

        const font: Font = Font {
            start: #start,
            end: #end,
            size: #size,
            height: #height,
            chars: vec![
                #(#tokens)*
            ],
        };
    };

    // Generate file
    let mut f = File::create(&config.output).map_err(|e| e.to_string())?;
    writeln!(f, "{}", font_map).map_err(|e| e.to_string())?;

    // Format output
    //Command::new("cargo fmt")
    //    .args(&["--", &config.output])
    //    .output()
    //    .map_err(|e| e.to_string())?;

    Ok(())
}
