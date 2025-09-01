# Asciinema Agg Python Bindings

This is a Python binding for [agg](https://github.com/asciinema/agg) which is a command line tool for converting asciinema recordings into GIF video files.

It requires a modified version of agg (by making some modules public), which is included in this repository at `local_cargo_registry/agg`.

<!-- TODO: create documentation with readthedocs.io or github wiki -->

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

If you just want to use internally, for example, rendering a ptyprocess output:

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

# Print a cybergod style terminal text dump with cursor
for index, it in enumerate(terminal_dump):
    if index == cursor_states[1]:
        it = it[:cursor_states[0]] +"<|cursor|>"+it[cursor_states[0]:]
    print(">", it)
```

Or render external terminal emulator screenshots, like tmux:

```python
# First, dump raw terminal buffer to a file:
#   tmux capture-pane -t <session_name> -p -e > terminal_dump.bin
# Then get terminal size and cursor location:
#   tmux list-session -F "[#{session_name}] socket: #{socket_path} size: #{window_width}x#{window_height} cursor at: x=#{cursor_x},y=#{cursor_y} cursor flag: #{cursor_flag} cursor character: #{cursor_character} insert flag: #{insert_flag}, keypad cursor flag: #{keypad_cursor_flag}, keypad flag: #{keypad_flag}" -f "#{==:#{session_name},<session_name>}" > session_info.txt
# Reference (TmuxSession > get_info):
#   https://github.com/James4Ever0/agi_computer_control/blob/master/tmux_trials%2Flib_stable.py#L532

import agg_python_bindings
import parse

with open("terminal_dump.bin", "rb") as f:
    terminal_bytes = f.read()
    terminal_string = terminal_bytes.decode("utf-8", errors="replace")

with open("session_info.txt", "r") as f:
    session_info = f.read().strip()

def parse_session_info(session_info: str) -> dict:
    numeric_properties = [
        "window_width",
        "window_height",
        "cursor_x",
        "cursor_y",
        "cursor_flag",
        "insert_flag",
        "keypad_cursor_flag",
        "keypad_flag",
    ]
    parse_format = list_session_format_template.replace(
        "#{", "{"
    )
    data = parse.parse(parse_format, session_info)
    if isinstance(data, parse.Result):
        ret = data.named
        for it in numeric_properties:
            ret[it] = int(ret[it])
        ret = dict(ret)
        return ret
    else:
        raise ValueError("Failed to parse session info")

session_info_parsed = parse_session_info(session_info)

cols = session_info_parsed["window_width"]
rows = session_info_parsed["window_height"]
cursor_col = session_info_parsed["cursor_x"]
cursor_row = session_info_parsed["cursor_y"]

terminal = agg_python_bindings.TerminalEmulator(cols=cols, rows=rows)
terminal.feed_str(terminal_string)
terminal.set_cursor(col=cursor_col, row=cursor_row)
terminal.screenshot("tmux_screenshot.png")
```