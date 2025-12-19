import signal
import curses
import time
import sys

is_defined = lambda var: var in globals() or var in locals()

try:
    if not is_defined("alphabet_large"):
        from alphabet import alphabet_large
    if not is_defined("alphabet_small"):
        from alphabet5 import alphabet_small
except ImportError:
    print("Error: Could not import alphabet modules.")
    sys.exit(1)


# =========================
# USER CONTROLS
# =========================
SCROLL_LOOPS = 5  # 0 = infinite
EAT_CTRL_C = True  # False = allow KeyboardInterrupt
SPEED = 0.05  # seconds per frame
SPEED = 0.02
TEXT = "MONSIEUR\n  JBB\nA L'AIDE!"
FONT_SIZE = "large"  # "large" or "small"

if len(sys.argv) > 1:
    if len(sys.argv) == 2:
        SCROLL_LOOPS = 1
        TEXT = sys.argv[1].replace("_", " ").replace("\\n", "\n").upper()
    else:
        if len(sys.argv) != 6:
            print(
                "Usage: python main.py <speed: seconds per frame; 0.02> <loops: 0=infinite> <eat_ctrl_c: 0 or 1> <font_size: large or small> '<text: use _ for spaces>'"
            )
            sys.exit(1)
        SPEED = float(sys.argv[1])
        SCROLL_LOOPS = int(sys.argv[2])
        EAT_CTRL_C = bool(int(sys.argv[3]))
        FONT_SIZE = sys.argv[4].lower()
        TEXT = sys.argv[5].replace("_", " ").replace("\\n", "\n").upper()


TEXT_HEIGHT = 0
LINES_SEP_COUNT = 0
ALPHABET = {}
if FONT_SIZE == "large":
    TEXT_HEIGHT = alphabet_large["TEXT_HEIGHT"]
    LINES_SEP_COUNT = alphabet_large["LINES_SEP_COUNT"]
    ALPHABET = alphabet_large["ALPHABET"]
elif FONT_SIZE == "small":
    TEXT_HEIGHT = alphabet_small["TEXT_HEIGHT"]
    LINES_SEP_COUNT = alphabet_small["LINES_SEP_COUNT"]
    ALPHABET = alphabet_small["ALPHABET"]
else:
    print("FONT_SIZE must be 'large' or 'small'")
    sys.exit(1)
# =========================
# RENDERING
# =========================


def render_big_text(text: str):
    big_lines = []
    lines = [""] * TEXT_HEIGHT
    for char in text.upper():
        if char == "\n":
            lines.extend(
                [""] * LINES_SEP_COUNT
            )  # Blank line between lines of text
            big_lines.extend(lines)
            lines = [""] * TEXT_HEIGHT
            continue
        glyph = ALPHABET.get(char, ALPHABET[" "])
        for i in range(TEXT_HEIGHT):
            lines[i] += glyph[i] + "  "
    big_lines.extend(lines)
    return big_lines


def safe_addstr(win, y, x, s):
    h, w = win.getmaxyx()
    if y < 0 or y >= h or x >= w:
        return
    if x < 0:
        s = s[-x:]
        x = 0
    if not s:
        return
    win.addstr(y, x, s[: w - x])


# =========================
# MAIN LOOP
# =========================


def main(stdscr):
    curses.curs_set(0)
    stdscr.nodelay(True)

    big_text = render_big_text(TEXT)
    text_width = len(big_text[0])

    loops_done = 0
    height, width = stdscr.getmaxyx()
    if len(big_text) > height:
        print("Error: Terminal height too small for the text.")
        sys.exit(1)
    x = width

    while SCROLL_LOOPS == 0 or loops_done < SCROLL_LOOPS:
        stdscr.erase()
        height, width = stdscr.getmaxyx()
        y = height // 2 - len(big_text) // 2

        for i, line in enumerate(big_text):
            safe_addstr(stdscr, y + i, x, line)

        stdscr.refresh()
        x -= 1

        if x < -text_width:
            x = width
            loops_done += 1

        time.sleep(SPEED)

        if stdscr.getch() != -1 and not EAT_CTRL_C:
            break


if __name__ == "__main__":
    if EAT_CTRL_C:
        # ACTUALLY eat Ctrl-C
        signal.signal(signal.SIGINT, signal.SIG_IGN)
        curses.wrapper(main)
    else:
        # Let Ctrl-C raise KeyboardInterrupt
        curses.wrapper(main)
