from typing import Optional
from sys import argv
import subprocess
import re

PADDING: int = 3

def is_neofetch_installed() -> bool:
    '''
    Checks if the 'neofetch' command-line utility is installed on the system.
    Returns:
        bool: True if 'neofetch' is found in the system's PATH, False otherwise.
    '''
    
    result: subprocess.CompletedProcess = subprocess.run(
        ['which', 'neofetch'],
        capture_output=True,
        text=True,
        shell=False)

    return result.returncode == 0

def run_neofetch() -> str:
    '''
    Executes the 'neofetch' command with the '--off' flag and captures its output.
    Returns:
        str: The standard output of the 'neofetch' command.
    '''

    result: subprocess.CompletedProcess = subprocess.run(
        ['neofetch', '--off'],
        capture_output=True,
        text=True,
        shell=False)

    output: str = result.stdout
    return output

def replace_all_colors_in_info(neofetch_info: str, color: tuple[int, int, int]) -> str:
    '''
    Replaces all colors in the given neofetch information string with a specified RGB color.
    This function takes a neofetch information string, removes any existing color codes, 
    and applies a new color to the user, host, and keys in the information. The values 
    remain uncolored.
    Args:
        neofetch_info (str): The neofetch information string to process.
        color (tuple[int, int, int]): A tuple representing the RGB color to apply 
            (red, green, blue).
    Returns:
        str: The neofetch information string with the new color applied.
    '''

    red: int = color[0]
    green: int = color[1]
    blue: int = color[2]
    color_code: str = f'\x1b[38;2;{red};{green};{blue}m'
    
    neofetch_info_nocolor: str = strip_color(neofetch_info)

    lines: list[str] = neofetch_info_nocolor.splitlines()

    new_neofetch_info_colored: str = ''

    firstLine: str = lines[0]
    userhost_parts: list[str] = firstLine.split('@')
    user: str = color_code + userhost_parts[0] + '\x1b[0m'
    host: str = color_code + userhost_parts[1] + '\x1b[0m'
    new_neofetch_info_colored += f'{user}@{host}\n'

    new_neofetch_info_colored += f'{lines[1]}\n'

    for i in range(2, len(lines[:-6])):
        line: str = lines[i]
        line_parts: list[str] = line.split(':', 1)
        info_key: str = color_code + line_parts[0] + '\x1b[0m'
        info_value: str = line_parts[1].lstrip()
        new_neofetch_info_colored += f'{info_key}: {info_value}\n'

    new_neofetch_info_colored += f'\n\b{neofetch_info.splitlines()[-5]}\n'
    new_neofetch_info_colored += f'\b{neofetch_info.splitlines()[-4]}\n'

    return new_neofetch_info_colored

def get_largest_line_length(ascii_art: str) -> int:
    '''
    Calculates the length of the longest line in the given ASCII art string.
    Args:
        ascii_art (str): A string containing ASCII art, where each line may have varying lengths.
    Returns:
        int: The length of the longest line in the ASCII art.
    '''

    lines: list[str] = ascii_art.splitlines()
    largest_length: int = max(len(line) for line in lines)
    return largest_length

def strip_color(ascii_art: str) -> str:
    '''
    Removes ANSI escape sequences used for color formatting from the given string.
    Args:
        ascii_art (str): A string containing ANSI art (probably with color).
    Returns:
        str: The input string with all ANSI escape sequences removed.
    '''

    ansi_escape: re.Pattern = re.compile(r'\x1b\[[0-9;]*m')
    return ansi_escape.sub('', ascii_art)

def strip_trailing_whitespace(ascii_art: str) -> str:
    '''
    Removes trailing whitespace from each line in the given ASCII art string.
    Args:
        ascii_art (str): A string containing ASCII art, where each line may have trailing whitespace.
    Returns:
        str: A new string with trailing whitespace removed from each line, preserving the original line structure.
    '''

    lines: list[str] = ascii_art.splitlines()
    stripped_lines: list[str] = [line.rstrip() for line in lines]
    return '\n'.join(stripped_lines)

def print_custom_neofetch(ascii_art: str, text_color: tuple[int, int, int], padding: int) -> None:
    '''
    Prints a custom neofetch output with ASCII art and text color customization.
    This function combines ASCII art with the output of the `neofetch` command, 
    replacing all colors in the neofetch output with the specified text color. 
    It aligns the ASCII art and the neofetch information side by side with a 
    specified padding.
    Args:
        ascii_art (str): The ASCII art to display alongside the neofetch output.
        text_color (tuple[int, int, int]): The RGB color to apply to the neofetch text.
        padding (int): The number of spaces to add between the ASCII art and the neofetch output.
    Returns:
        None
    '''

    neofetch_output: str = run_neofetch()
    neofetch_custom_info: str = replace_all_colors_in_info(neofetch_output, text_color)
    ascii_art_no_color: str = strip_color(ascii_art)
    ascii_art_no_color = strip_trailing_whitespace(ascii_art_no_color)

    ascii_art_lines: list[str] = ascii_art.splitlines()
    ascii_art_no_color_lines: list[str] = ascii_art_no_color.splitlines()
    neofetch_custom_info_lines: list[str] = neofetch_custom_info.splitlines()

    for i in range(max(len(ascii_art_no_color_lines), len(neofetch_custom_info_lines))):
        ascii_art_line: Optional[str] = ascii_art_lines[i] if i < len(ascii_art_lines) else None
        neofetch_info_line: Optional[str] = neofetch_custom_info_lines[i] if i < len(neofetch_custom_info_lines) else None

        if ascii_art_line is not None:
            ascii_art_line += ' ' * padding
        else:
            ascii_art_line = ' ' * get_largest_line_length(ascii_art_no_color) + ' ' * padding

        if ascii_art_line is None: ascii_art_line = ''
        if neofetch_info_line is None: neofetch_info_line = ''

        print(f'{ascii_art_line}{neofetch_info_line}')

def main():
    '''
    Main function to execute the custom Neofetch script.
    This script checks if Neofetch is installed, validates the command-line arguments,
    reads an ASCII art file, and displays it with optional text color customization.
    Usage:
        python neofetch_custom.py <file with ascii art>
        Flags:
        --txtclr <r> <g> <b> - Color the text with the specified RGB color values (0-255 each), default is white.
    Steps:
    1. Verifies if Neofetch is installed.
    2. Validates the number and format of command-line arguments.
    3. Reads the ASCII art from the specified file.
    4. Displays the ASCII art with the specified or default text color.
    5. Resets the terminal state to prevent display issues.
    Raises:
        ValueError: If RGB color values are not integers between 0 and 255.
        Exception: If an error occurs while checking for Neofetch installation.
    '''

    try:
        if not is_neofetch_installed():
            print('Error: Neofetch is not installed. Please install Neofetch to use this script.')
            return
    except Exception as e:
        print(f'Error checking for Neofetch installation: {e}')
        return

    if not (len(argv) == 2 or len(argv) == 6):
        print('Usage: python neofetch_custom.py <file with ascii art>\n\nFlags:\n--txtclr <r> <g> <b> - Color the text with the specified RGB color values (0-255 each), default is white')
        return
    
    ascii_art_file: str = argv[1]
    text_color: tuple[int, int, int] = (255, 255, 255)

    try:
        if len(argv) > 2:
            if argv[2] != '--txtclr':
                print(f'Error: Invalid arguments. Unknown flag {argv[2]} provided.')
                return
            
            if int(argv[3]) < 0 or int(argv[3]) > 255 or int(argv[4]) < 0 or int(argv[4]) > 255 or int(argv[5]) < 0 or int(argv[5]) > 255:
                raise ValueError()
        
            text_color = (int(argv[3]), int(argv[4]), int(argv[5]))
    except ValueError:
        print('Error: RGB color values must be integers between 0 and 255.')
        return
        
    with open(ascii_art_file, 'r') as file:
        ascii_art: str = file.read()

    print_custom_neofetch(ascii_art, text_color, padding=PADDING)
    print('\n')

    # Reset terminal state to prevent issues with command history display
    print('\033[?7h', end='')  # Enable line wrapping
    print('\033[?25h', end='') # Show cursor
    print('\033[0m', end='')   # Reset all formatting

if __name__ == '__main__':
    main()
