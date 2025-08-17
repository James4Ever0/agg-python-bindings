import agg_python_bindings

def test_counter_class_rust_usage():
    # Create an instance
    counter = agg_python_bindings.Counter(10)

    # Access state
    print(counter.get_count())  # Output: 10

    # Modify state
    counter.increment()
    print(counter.get_count())  # Output: 11

    # Use methods with error handling
    counter.add(5)  # New count: 16
    print(counter)

    # Handle errors
    try:
        counter.add(2**30)  # Triggers overflow
    except ValueError as e:
        print(f"Error: {e}")  # Output: Error: Overflow error!

def test_asciicast_to_png_screenshot():
    import os

    test_file = "../agi_computer_control/web_gui_terminal_recorder/record_viewer/vim_rectangle_selection_test/vim_rectangle_selection.cast"
    png_output_path = "./png_output_test"
    print("Asciicast test file: " + test_file)
    print("PNG output path: " + png_output_path)
    os.makedirs(png_output_path, exist_ok=True)
    if os.path.exists(test_file):
        print("File exists")
        agg_python_bindings.load_asciicast_and_save_png_screenshots(test_file, png_write_dir=png_output_path)
    else:
        print("File does not exist")

def test_virtual_terminal():
    test_input = 'Hello from \x1B[1;3;31mxterm.js\x1B[0m $ '
    screenshot_path = "test_helloworld_screenshot.png"
    terminal = agg_python_bindings.TerminalEmulator(80, 25)
    changed = terminal.feed_str(test_input)
    width, height, success = terminal.screenshot(screenshot_path)
    print("Terminal changed:", changed)
    print("Terminal size: width=%s, height=%s" % (width, height))
    print("Screenshot success:", success)
    print("View screenshot at:", screenshot_path)

def test():
    print("Testing asciicast to png")
    test_asciicast_to_png_screenshot()
    # print()
    # print("Testing rust class: counter")
    # test_counter_class_rust_usage() # working, the thing increases.
    print()
    print("Testing virtual terminal class") # screenshot working
    test_virtual_terminal()

if __name__ == "__main__":
    test()
