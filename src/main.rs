use chrono::*;
use ncurses::WchResult::KeyCode;
use ncurses::*;
use num_traits::cast::FromPrimitive;
use std::cmp;

mod date_format;
mod key;

type Pair = i16;

const REGULAR_PAIR: Pair = 0;
const HIGHLIGHT_PAIR: Pair = 1;
const REGULAR_RED_PAIR: Pair = 2;
const HIGHLIGHT_RED_PAIR: Pair = 3;
const REGULAR_GREEN_PAIR: Pair = 4;
const HIGHLIGHT_GREEN_PAIR: Pair = 5;

#[derive(Default)]
struct Ui {
    row: usize,
    col: usize,
    width: usize,
}

impl Ui {
    fn begin(&mut self, row: usize, col: usize) {
        self.row = row;
        self.col = col;
        self.width = 0;
    }
    fn label(&mut self, text: &str, pair: Pair) {
        self.label_fix_width(text, pair, text.len())
    }
    fn label_fix_width(&mut self, text: &str, pair: Pair, _width: usize) {
        if self.width < _width {
            self.width = _width;
        }
        mv(self.row as i32, self.col as i32);
        attron(COLOR_PAIR(pair));
        addstr(text);
        attroff(COLOR_PAIR(pair));
        self.row += 1;
    }
    fn label_center(&mut self, text: &str, pair: Pair) {
        self.width = cmp::max(self.width, text.len());
        let space = (21_usize - text.len()) / 2;
        mv(self.row as i32, self.col as i32 + space as i32);
        attron(COLOR_PAIR(pair));
        addstr(text);
        attroff(COLOR_PAIR(pair));
        self.row += 1;
    }

    fn add_str(&mut self, text: &str, pair: Pair, space: usize) {
        mv(self.row as i32, self.col as i32);
        attron(COLOR_PAIR(pair));
        addstr(text);
        attroff(COLOR_PAIR(pair));
        self.col += text.len() + space;
    }
    fn end(&mut self) {}
}

fn main() {
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(REGULAR_GREEN_PAIR, COLOR_GREEN, COLOR_BLACK);
    init_pair(REGULAR_RED_PAIR, COLOR_RED, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);
    init_pair(HIGHLIGHT_GREEN_PAIR, COLOR_BLACK, COLOR_GREEN);
    init_pair(HIGHLIGHT_RED_PAIR, COLOR_BLACK, COLOR_RED);

    let mut quit = false;

    let mut ui = Ui::default();
    while !quit {
        erase();
        let now = Local::now();
        let month = chrono::Month::from_u32(now.month()).unwrap();

        let start_day = date_format::weekday_to_index(now.with_day(1).unwrap().weekday());
        let curr_day = now.day();

        let month_len = date_format::month_length(now.month(), date_format::leap_year(now.year()));
        ui.begin(0, 0);
        {
            ui.label_center(&format!("{} {}", month.name(), now.year()), REGULAR_PAIR);
            ui.label("So Mo Di Mi Do Fr Sa", REGULAR_PAIR);

            {
                let mut found_start = false;
                let mut i = 1;
                let mut j = 1;
                while month_len >= i {
                    if !found_start && start_day == j {
                        found_start = true;
                    }
                    if !found_start {
                        ui.add_str("  ", REGULAR_PAIR, 1);
                        j += 1;
                        continue;
                    }
                    if i < 10 && i == curr_day {
                        ui.add_str(&format!("0{}", i), HIGHLIGHT_PAIR, 1);
                    } else if i > 10 && i == curr_day {
                        ui.add_str(&format!("{}", i), HIGHLIGHT_PAIR, 1);
                    } else if i < 10 {
                        ui.add_str(&format!("0{}", i), REGULAR_PAIR, 1);
                    } else {
                        ui.add_str(&format!("{}", i), REGULAR_PAIR, 1);
                    }
                    if j % 7 == 0 {
                        ui.row += 1;
                        ui.col = 0;
                    }
                    j += 1;
                    i += 1;
                }
            }
        }
        ui.end();
        refresh();

        let key = getch();
        match key {
            key::Q => {
                quit = true;
            }
            _ => {
                addstr(&format!("{:?}", KeyCode(key)));
                getch();
            }
        }
    }

    endwin();
}
