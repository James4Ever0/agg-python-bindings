# Asciinema Agg Python Bindings

This is a Python binding for [agg](https://github.com/asciinema/agg) which is a command line tool for converting asciinema recordings into GIF video files.

It requires a modified version of agg (by making some modules public), which is included in this repository at `local_cargo_registry/agg`.

## Installation

From PyPI:

```bash
pip install agg-python-bindings
```

From GitHub:

```bash
pip install git+https://github.com/james4ever0/agg-python-bindings.git
```

Source install:

```bash
git clone https://github.com/james4ever0/agg-python-bindings.git
cd agg-python-bindings
pip install .
```

## Usage

```python
import agg_python_bindings

asciicast_filepath = "asciinema.cast"

# Load asciicast file from path, save terminal screenshots separated by frame_time_min_spacing (seconds) to png_write_dir
# Output png filename format: "{png_filename_prefix}_{screenshot_timestamp}.png"
agg_python_bindings.load_asciicast_and_save_png_screenshots(
    asciicast_filepath, # required, path to asciicast file (input)
    png_write_dir=".", # optional
    png_filename_prefix="screenshot", # optional
    frame_time_min_spacing=1.0 # optional
)
```