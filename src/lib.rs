///
/// Pex Challenge
/// Samuel Lewis, January 2018
///

use clap::{Arg, App};
use image::Pixel;
use image::GenericImageView;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;


/// Frame structure to represent output of ffprobe command
///
/// A better solution would impl Serde deserilize for the data, or simply provide proper FFI bindings
/// for the ffmpeg C library. Not used for toy case to reduce dev time. The existing FFI bindings
/// for ffmpeg are extremely stale, so they were not used.
///
/// Also chose to skip properly parsing types, as most of this is unused.
#[derive(Debug)]
pub struct Frame {
    pub media_type: String,
    pub stream_index: String,
    pub key_frame: String, // 1 = true
    pub pkt_pts: String,
    pub pkt_pts_time: String,
    pub pkt_dts: String,
    pub pkt_dts_time: String,
    pub best_effort_timestamp: String,
    pub best_effort_timestamp_time: String,
    pub pkt_duration: String,
    pub pkt_duration_time: String,
    pub pkt_pos: String,
    pub pkt_size: String,
    pub width: String,
    pub height: String,
    pub pix_fmt: String,
    pub sample_aspect_ratio: String,
    pub pict_type: String,
    pub coded_picture_number: String,
    pub display_picture_number: String,
    pub interlaced_frame: String,
    pub top_field_first: String,
    pub repeat_pict: String,
    pub color_range: String,
    pub color_space: String,
    pub color_primaries: String,
    pub color_transfer: String,
    pub chroma_location: String,
}

#[derive(Debug)]
pub struct Frames(Vec<Frame>);

impl From<File> for Frames {
    fn from(file: File) -> Frames {
        let mut frames: Vec<Frame> = Vec::new();
        let mut lines = BufReader::new(file).lines();
        loop { // Loop over frames fields
            if let Some(_) = lines.next() { // If there is a [Frame] statement               
                frames.push(Frame { // This is awful style, brittle and hard to debug, only good for toy case
                    media_type: lines.next().unwrap().unwrap().split('=').collect::<Vec<&str>>()[1].to_string(),
                    stream_index: lines.next().unwrap().unwrap().split('=').collect::<Vec<&str>>()[1].to_string(),
                    key_frame: lines.next().unwrap().unwrap().split('=').collect::<Vec<&str>>()[1].to_string(),
                    pkt_pts: lines.next().unwrap().unwrap().split('=').collect::<Vec<&str>>()[1].to_string(),
                    pkt_pts_time: lines.next().unwrap().unwrap().split('=').collect::<Vec<&str>>()[1].to_string(),
                    pkt_dts: lines.next().unwrap().unwrap().split('=').collect::<Vec<&str>>()[1].to_string(),
                    pkt_dts_time: lines.next().unwrap().unwrap().split('=').collect::<Vec<&str>>()[1].to_string(),
                    best_effort_timestamp: lines.next().unwrap().unwrap().split('=').collect::<Vec<&str>>()[1].to_string(),
                    best_effort_timestamp_time: lines.next().unwrap().unwrap().split('=').collect::<Vec<&str>>()[1].to_string(),
                    pkt_duration: lines.next().unwrap().unwrap().split('=').collect::<Vec<&str>>()[1].to_string(),
                    pkt_duration_time: lines.next().unwrap().unwrap().split('=').collect::<Vec<&str>>()[1].to_string(),
                    pkt_pos: lines.next().unwrap().unwrap().split('=').collect::<Vec<&str>>()[1].to_string(),
                    pkt_size: lines.next().unwrap().unwrap().split('=').collect::<Vec<&str>>()[1].to_string(),
                    width: lines.next().unwrap().unwrap().split('=').collect::<Vec<&str>>()[1].to_string(),
                    height: lines.next().unwrap().unwrap().split('=').collect::<Vec<&str>>()[1].to_string(),
                    pix_fmt: lines.next().unwrap().unwrap().split('=').collect::<Vec<&str>>()[1].to_string(),
                    sample_aspect_ratio: lines.next().unwrap().unwrap().split('=').collect::<Vec<&str>>()[1].to_string(),
                    pict_type: lines.next().unwrap().unwrap().split('=').collect::<Vec<&str>>()[1].to_string(),
                    coded_picture_number: lines.next().unwrap().unwrap().split('=').collect::<Vec<&str>>()[1].to_string(),
                    display_picture_number: lines.next().unwrap().unwrap().split('=').collect::<Vec<&str>>()[1].to_string(),
                    interlaced_frame: lines.next().unwrap().unwrap().split('=').collect::<Vec<&str>>()[1].to_string(),
                    top_field_first: lines.next().unwrap().unwrap().split('=').collect::<Vec<&str>>()[1].to_string(),
                    repeat_pict: lines.next().unwrap().unwrap().split('=').collect::<Vec<&str>>()[1].to_string(),
                    color_range: lines.next().unwrap().unwrap().split('=').collect::<Vec<&str>>()[1].to_string(),
                    color_space: lines.next().unwrap().unwrap().split('=').collect::<Vec<&str>>()[1].to_string(),
                    color_primaries: lines.next().unwrap().unwrap().split('=').collect::<Vec<&str>>()[1].to_string(),
                    color_transfer: lines.next().unwrap().unwrap().split('=').collect::<Vec<&str>>()[1].to_string(),
                    chroma_location: lines.next().unwrap().unwrap().split('=').collect::<Vec<&str>>()[1].to_string()
                });
            } else {
                break;
            }
            let _ = lines.next(); // Discard [/Frame] statement
        }
        Frames(frames)
    }
}

pub fn main() {

    // handle case where width/height exceed dimensions

    let matches = App::new("Pex Challenge Input")
        .author("Samuel Lewis <samuellewis828@gmail.com>")
        .about("Quick and dirty code for Pex Coding Challenge")
        .arg(Arg::with_name("width")
            .short("w")
            .long("width")
            .help("Width of grid to split frames into, must be greater than zero and less than or equal to frame width.")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("height")
            .short("h")
            .long("height")
            .help("Height of grid to split frames into, must be greater than zero and less than or equal to frame height.")
            .required(true)
            .takes_value(true))
        .get_matches();

    let provided_width: u32 = matches.value_of("width").expect("No width value provided").parse().unwrap();
    let provided_height: u32 = matches.value_of("height").expect("No height value provided").parse().unwrap();

    if provided_width == 0 || provided_height == 0 {
        panic!("Provided width or height was zero, and values must be greater than zero.");
    }


    let file: File = File::open("frames.txt").expect("Failed to open expected frames.txt");
    let frames: Frames = file.into();

    let mut output = File::create("output.csv").unwrap();

    println!("Looking at {} frames.", frames.0.len());
    if frames.0.len() == 0 {
        panic!("Frames was 0");
    }

    for index in 0..frames.0.len() {
        // Read in images
        let image = &image::open(&Path::new(&format!("output_pictures/thumbnails-{}.png", index + 1))).unwrap();

        // For dividing, I just take requested size blocks until I can't - missing pixels get lost
        // Instructions don't specify a behavior for edge cases, so I took the easy path
        let width = image.width();
        let height = image.height();

        if width < provided_width { panic!("Provided width must be less than or equal to the total frame width."); };
        if height < provided_height { panic!("Provided height must be less than or equal to the total frame height."); };


        output.write_all(format!("{}", frames.0[index].best_effort_timestamp_time).as_bytes()).unwrap();
        // Go over width, just use floored value to avoid getting not enough pixels
        for w_index in 0..provided_width {
            // Go over height, again use floored value to avoid not enough pixels
            for h_index in 0..provided_height {
                // Borrow a sub view of the image, get median pixel value
                let sub_width = width / provided_width;
                let sub_height = height / provided_height;
                let sub_image = image.view(w_index * sub_width, h_index * sub_height, sub_width, sub_height);
                let mut pixel_vals: Vec<u8> = sub_image.pixels().map(|pixel| pixel.2.to_luma()[0]).collect::<Vec<u8>>();
                pixel_vals.sort();
                let median = if pixel_vals.len() % 2 == 0 { // even - take both "middle values" get their average
                    (((pixel_vals[pixel_vals.len() / 2] as u32) + (pixel_vals[(pixel_vals.len() / 2) - 1]) as u32) / 2) as u8 // prevents possible overflow
                } else { // odd
                    pixel_vals[pixel_vals.len() / 2 as usize]
                };
                output.write_all(format!(",{}", median).as_bytes()).unwrap();
            }
        }
        output.write_all(b"\n").unwrap();
    }
    println!("Check output.csv for the results.");


    // TODO
    // Did split logic wrong - 3x3 means give back 9 cells, i take as many 3x3 cells as possible...
}



















