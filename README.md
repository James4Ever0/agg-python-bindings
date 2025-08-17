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

# Create a virtual terminal, feed string into it, then get states and take a screenshot
test_input = 'Hello from \x1B[1;3;31mxterm.js\x1B[0m $ '
screenshot_path = "terminal_screenshot.png"
terminal = agg_python_bindings.TerminalEmulator(80, 25)
changed = terminal.feed_str(test_input)
terminal_dump = terminal.text_raw()
cursor_states = terminal.get_cursor()
width, height, success = terminal.screenshot(screenshot_path)

# print a cybergod style terminal text dump with cursor
for index, it in enumerate(terminal_dump):
    if index == cursor_states[1]:
        it = it[:cursor_states[0]] +"<|cursor|>"+it[cursor_states[0]:]
    print(">", it)
```