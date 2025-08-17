use agg;
use agg::asciicast;
use agg::renderer::Renderer;
use image;
use pyo3::prelude::*;
use std::path::Path;

fn create_default_renderer(
    terminal_size: (usize, usize),
) -> agg::renderer::fontdue::FontdueRenderer {
    let config_font_dirs = vec![];
    let config_font_family = String::from(agg::DEFAULT_FONT_FAMILY);

    let (font_db, font_families) =
        agg::fonts::init(&config_font_dirs, &config_font_family).unwrap();
    let font_size = agg::DEFAULT_FONT_SIZE;
    let line_height = agg::DEFAULT_LINE_HEIGHT;

    let renderer_settings = agg::renderer::Settings {
        terminal_size: terminal_size,
        font_db: font_db,
        font_families: font_families,
        font_size: font_size,
        line_height: line_height,
        theme: agg::Theme::Dracula.try_into().unwrap(),
    };

    let renderer = agg::renderer::fontdue(renderer_settings);

    renderer
}

#[pyfunction]
#[pyo3(signature = (cast_file_loadpath, *, png_write_dir=".".to_string(), png_filename_prefix="screenshot".to_string(), frame_time_min_spacing=1.0, verbose=false))]
fn load_asciicast_and_save_png_screenshots(
    cast_file_loadpath: String,
    png_write_dir: String,
    png_filename_prefix: String,
    frame_time_min_spacing: f64,
    verbose: bool,
) -> PyResult<()> {
    assert_is_directory(&png_write_dir);

    let file_io = std::fs::File::open(cast_file_loadpath)?;
    let input = std::io::BufReader::new(file_io);
    let (header, events) = asciicast::open(input).unwrap();

    let terminal_size = (header.terminal_size.0, header.terminal_size.1);
    let mut renderer = create_default_renderer(terminal_size);
    if verbose {
        // print header
        println!(
            "Header: idie_time_limit={:?}, terminal_size={:?}, theme={:?}",
            header.idle_time_limit, header.terminal_size, header.theme
        );

        // print terminal size
        println!("Terminal size: {:?}", terminal_size);
    }

    let out_events = agg::asciicast::output(events);

    let frames = agg::vt::frames(out_events, terminal_size);

    let mut last_frame_time: f64 = 0.0;

    for frame in frames {
        let (time, lines, cursor) = frame;
        if verbose {
            println!("Rendering frame at time {time:}");
        }
        if time >= last_frame_time {
            last_frame_time = time + frame_time_min_spacing;
        } else {
            if verbose {
                println!(
                    "Skipping frame due to frame time min spacing {frame_time_min_spacing:} secs"
                );
            }
            continue;
        }
        let image = renderer.render(lines, cursor);
        let width = image.width();
        let height = image.height();
        let path = format!("{png_write_dir:}/{png_filename_prefix:}_{time:}.png");
        if verbose {
            println!("Image width: {width:}, height: {height:}");
            println!("Writing image at: {path:}");
        }

        save_to_png_rgba(image, &path).unwrap();
    }
    Ok(())
}

fn save_to_png_rgba(img: imgref::ImgVec<rgb::RGBA8>, path: &str) -> Result<(), image::ImageError> {
    let (width, height) = (img.width() as u32, img.height() as u32);
    let img_buf = image::RgbImage::from_fn(width, height, |x, y| {
        let pixel = img[(x as usize, y as usize)];
        image::Rgb([pixel.r, pixel.g, pixel.b])
    });
    img_buf.save(path)
}

#[pyclass]
struct TerminalEmulator {
    vt: agg::vt::avt::Vt,
    renderer: agg::renderer::fontdue::FontdueRenderer,
    prev_cursor: Option<(usize, usize)>,
}

#[pymethods]
impl TerminalEmulator {
    #[new]
    fn new(cols: usize, rows: usize) -> Self {
        TerminalEmulator {
            vt: agg::vt::avt::Vt::builder()
                .size(cols, rows)
                .scrollback_limit(0)
                .build(),
            renderer: create_default_renderer((cols, rows)),
            prev_cursor: None,
        }
    }
    fn feed_str(&mut self, data: String) -> bool {
        // return value: visually changed or not
        let changed_lines = self.vt.feed_str(&data).lines;
        let cursor: Option<(usize, usize)> = self.vt.cursor().into();

        let has_change = !changed_lines.is_empty() || cursor != self.prev_cursor;

        self.prev_cursor = cursor;
        has_change
    }
    fn text(&self) -> Vec<String> {
        self.vt.text()
    }

    fn text_raw(&self) -> Vec<String> {
        let mut ret = Vec::new();
        for line in self.vt.lines() {
            let it = line.text();
            ret.push(it)
        }
        ret
    }

    fn get_cursor(&self) -> (usize, usize, bool) {
        let cursor = self.vt.cursor();
        (cursor.col, cursor.row, cursor.visible)
    }

    fn screenshot(&mut self, png_output_path: String) -> (usize, usize, bool) {
        let lines = self.vt.lines();
        let cursor = self.vt.cursor();
        let image = self.renderer.render(lines.into(), cursor.into());
        let width = image.width();
        let height = image.height();
        let ok = save_to_png_rgba(image, &png_output_path).is_ok();
        (width, height, ok)
    }
}

#[pymodule]
fn agg_python_bindings(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(
        load_asciicast_and_save_png_screenshots,
        m
    )?)?;
    m.add_class::<TerminalEmulator>()?;
    Ok(())
}

fn assert_is_directory(path: &str) {
    let path = Path::new(path);

    // Check if the path exists and is accessible
    let metadata = path.metadata().unwrap_or_else(|_| {
        panic!("Path `{}` does not exist", path.display());
    });

    // Verify the path points to a directory
    if !metadata.is_dir() {
        panic!("Path `{}` is not a directory", path.display());
    }
}
