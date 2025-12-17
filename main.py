import signal
import curses
import time
from alphabet import ALPHABET, TEXT_HEIGHT

# =========================
# USER CONTROLS
# =========================

SCROLL_LOOPS = 1  # 0 = infinite
EAT_CTRL_C = True  # False = allow KeyboardInterrupt
SPEED = 0.05  # seconds per frame
SPEED = 0.02

TEXT = "MONSIEUR\n  JBB\nA L'AIDE"

# =========================
# RENDERING
# =========================


def render_big_text(text: str):
    big_lines = []
    lines = [""] * TEXT_HEIGHT
    for char in text.upper():
        if char == "\n":
            lines.append("")  # Blank line between lines of text
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
