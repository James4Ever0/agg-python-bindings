# view example type annotation at: https://github.com/PyO3/maturin/blob/main/test-crates/pyo3-pure

# TODO: ingest arbitrary terminal sequence (like the one from tmux) and produce png output

# TODO: create a virtual terminal class with state, can take a screenshot at arbitrary state

# TODO: implement the function "load_asciicast_and_return_iterator", return iterator of events (str) with timestamp (float)

# TODO: customize render parameters, like fonts, theme, line height, etc.

def load_asciicast_and_save_png_screenshots(cast_file_loadpath: str, png_write_dir:str=".",png_filename_prefix:str="screenshot", frame_time_min_spacing:float=1.0, verbose=False) -> None:
    """
    Load asciicast file from path, save terminal screenshots separated by frame_time_min_spacing (seconds)

    Output png filename format: "{png_filename_prefix}_{screenshot_timestamp}.png"
    """
    ...
