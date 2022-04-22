use chrono::*;
use ncurses::WchResult::KeyCode;
use ncurses::*;
use num_traits::cast::FromPrimitive;
use std::cmp;
use num_traits::Zero;

mod date_format;
mod key;

type Pair = i16;

const REGULAR_PAIR: Pair = 0;
const REGULAR_RED_PAIR: Pair = 1;
const REGULAR_GREEN_PAIR: Pair = 2;
const HIGHLIGHT_PAIR: Pair = 3;
const HIGHLIGHT_RED_PAIR: Pair = 4;
const HIGHLIGHT_GREEN_PAIR: Pair = 5;
const SELECT_PAIR: Pair = 6;
const SELECT_RED_PAIR:Pair=7;
const SELECT_GREEN_PAIR: Pair = 8;

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

#[derive(PartialEq)]
enum Focus {
    Calendar,
    Date,
}

fn main() {
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    start_color();


    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(REGULAR_RED_PAIR, COLOR_RED, COLOR_BLACK);
    init_pair(REGULAR_GREEN_PAIR, COLOR_GREEN, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);
    init_pair(HIGHLIGHT_RED_PAIR, COLOR_BLACK, COLOR_RED);
    init_pair(HIGHLIGHT_GREEN_PAIR, COLOR_BLACK, COLOR_GREEN);
    init_pair(SELECT_PAIR, COLOR_WHITE, COLOR_CYAN);
    init_pair(SELECT_RED_PAIR, COLOR_RED, COLOR_CYAN);
    init_pair(SELECT_GREEN_PAIR, COLOR_GREEN, COLOR_CYAN);

    let mut quit = false;

    let mut ui = Ui::default();
    let mut days: Vec<Vec<bool>> = vec![vec![false; 7]; 6];
    let mut curs_x:u32 = 0;
    let mut curs_y:u32 = 0;

    let mut first = true;
    let mut date_changed = false;

    let mut month: u32 = 1;
    let mut year: i32 = 1;
    let mut curr_day:u32 = 0;

    let mut focus: Focus = Focus::Calendar;

    while !quit {
        erase();
        let now = Local::now();

        if first {
            month = now.month();
            year = now.year();
            curr_day = now.day();
            first = false;
        }

        let start_day = date_format::weekday_to_index(now.with_day(1).unwrap().weekday());
        let month_len = date_format::month_length(month, date_format::leap_year(year));

        ui.begin(0, 0);
        {
            let format_month = chrono::Month::from_u32(month).unwrap().name();
            if focus == Focus::Date {
                ui.label_center(&format!("{} {}", format_month, year), SELECT_PAIR);
            } else {
                ui.label_center(&format!("{} {}", format_month, year), REGULAR_PAIR);
            }
            ui.label("So Mo Di Mi Do Fr Sa", REGULAR_PAIR);

            {
                let mut found_start = false;
                let mut row = 1;
                let mut col = 1;
                let mut index = 1;
                while month_len >= index {


                    if !found_start && start_day == col{
                        found_start = true;
                    }
                    if !found_start {
                        ui.add_str("  ", REGULAR_PAIR, 1);
                        col += 1;
                        continue;
                    }

                    if date_changed {
                        curs_x = row;
                        curs_y = col;
                        date_changed = false;
                    }

                    if row == curs_x && col == curs_y && focus == Focus::Calendar{
                        ui.add_str(&format!("{:0>2}", index), SELECT_PAIR, 1)
                    } else if index == curr_day && month == now.month() && year == now.year() {
                        ui.add_str(&format!("{:0>2}", index), HIGHLIGHT_PAIR, 1);
                        if curs_y.is_zero() {
                            curs_x = row;
                            curs_y = col;
                        }
                    } else{
                        ui.add_str(&format!("{:0>2}", index), REGULAR_PAIR, 1);
                    }
                    days[(row - 1) as usize][(col - 1) as usize] = true;


                    if col == 7 {
                        ui.row += 1;
                        ui.col = 0;
                        col = 0;
                        row += 1;
                    }

                    col += 1;
                    index += 1;
                }
            }
        }
        ui.end();
        refresh();

        let key = getch();
        match key {
            key::Q => {
                quit = true;
            },
            key::ARROW_LEFT => {
                if curs_y > 1 && days[(curs_x - 1) as usize][(curs_y - 2) as usize] && focus == Focus::Calendar {
                    curs_y -= 1;
                }
                if focus == Focus::Date {
                    year -= 1;
                   date_changed = true;
                }
            },
            key::ARROW_RIGHT => {
                if curs_y < 7 && days[(curs_x - 1) as usize][curs_y as usize] && focus == Focus::Calendar{
                    curs_y += 1;
                }
                if focus == Focus::Date {
                    year += 1;
                    date_changed = true;
                }
            },
            key::ARROW_UP => {
                if curs_x > 1 && days[(curs_x - 2) as usize][(curs_y - 1) as usize] && focus == Focus::Calendar {
                    curs_x -= 1;
                }
                if focus == Focus::Date {
                    if month >= 12 {
                        month = 1;
                    } else {
                        month += 1;
                    }
                    date_changed = true;
                }
            },
            key::ARROW_DOWN => {
                if curs_x < 6 && days[curs_x as usize][(curs_y - 1) as usize] && focus == Focus::Calendar {
                    curs_x += 1;
                }
                if focus == Focus::Date {
                    if month <= 1 {
                        month = 12;
                    } else {
                        month -= 1;
                    }
                    date_changed = true;
                }
            },
            key::D => {
                focus = Focus::Date;
            },
            key::C => {
                focus = Focus::Calendar;
            }
            _ => {
                addstr(&format!("{:?}", KeyCode(key)));
                getch();
            }
        }
    }

    endwin();
}
