# view example type annotation at: https://github.com/PyO3/maturin/blob/main/test-crates/pyo3-pure

# TODO: ingest arbitrary terminal sequence (like the one from tmux) and produce png output

# TODO: create a virtual terminal class with state, can take a screenshot at arbitrary state

# TODO: implement the function "load_asciicast_and_return_iterator", return iterator of events (str) with timestamp (float)

# TODO: customize render parameters, like fonts, theme, line height, etc.

def load_asciicast_and_save_png_screenshots(
    cast_file_loadpath: str,
    png_write_dir: str = ".",
    png_filename_prefix: str = "screenshot",
    frame_time_min_spacing: float = 1.0,
    verbose=False,
) -> None:
    """
    Load asciicast file from path, save terminal screenshots separated by frame_time_min_spacing (seconds)

    Output png filename format: "{png_filename_prefix}_{screenshot_timestamp}.png"
    """
    

class TerminalEmulator:
    """
    A terminal emulator wrapper over avt, with output feeding, text dumping, cursor location and screenshot.
    """
    def __init__(self, cols: int, rows:int):
        """
        Create a virtual terminal with parameter determined size.
        
        Parameters:
            columns (cols: int)
            rows/lines (rows: int)
        """
        
    def feed_str(self, data:str) -> bool:
        """
        Feed data into the terminal, and get if the terminal has visible changes.

        Parameters:
            data: str

        Returns:
            changed: bool
        """
        
    def text(self)-> list[str]:
        """
        Iterate over terminal lines, convert into str, trim and remove empty ones, then return the final text list.
        """
        
    def text_raw(self)-> list[str]:
        """
        Iterate over terminal lines, convert into str, without trimming or empty check.
        """
        
    def get_cursor(self)-> tuple[int, int, bool]:
        """
        Get the current cursor properties.

        Returns:
            (col: int, row: int, visible: bool)
        """
        
    def screenshot(self, png_output_path:str) -> tuple[int, int, bool]:
        """
        Save the current terminal state as screenshot as PNG at given path.

        Returns:
            (width: int, height:int, success: bool)
        """
        