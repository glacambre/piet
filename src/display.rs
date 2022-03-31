use codel::*;
use pietcolor::*;
use std::*;
extern crate termion;

fn print_pic(picture: &Vec<Vec<Codel>>, offset_x: usize, offset_y: usize) {
    for row in picture.iter() {
        for codel in row.iter() {
            print!(
                "{}{}",
                termion::cursor::Goto(
                    (codel.x * 2 + offset_x + 1) as u16,
                    (codel.y + offset_y + 1) as u16,
                ),
                codel.color
            );
        }
        println!("{}\r", termion::style::Reset);
    }
}

fn display_pic(
    picture: Vec<Vec<Codel>>,
    receiving_end: sync::mpsc::Receiver<(bool, Codel, Direction, Direction)>,
) {
    use termion::raw::IntoRawMode;
    use std::io::Write;
    use std::io::Read;
    let _stdout = termion::input::MouseTerminal::from(io::stdout().into_raw_mode().unwrap());
    let mut screen = termion::screen::AlternateScreen::from(io::stdout());
    print!("{}", termion::cursor::Hide);

    let white = PietColor { hue: Hue::White, lightness: Lightness::Normal };
    while let Ok((true, codel, _, _)) = receiving_end.recv() {
        print_pic(&picture, 0, 0);

        let (x, y) = (codel.x as u16 * 2 + 1, codel.y as u16 + 2);
        print!("{}{}", termion::cursor::Goto(x, y), white);
        screen.flush().unwrap();

        thread::sleep(time::Duration::from_millis(250));
        print!("{}{}{}", termion::cursor::Goto(x, y), codel.color, termion::style::Reset);
        screen.flush().unwrap();

        thread::sleep(time::Duration::from_millis(250));
    }

    print!("{}", termion::cursor::Show);
    io::stdin().bytes().next();
    screen.flush().unwrap();
}

pub fn setup_display(
    picture: Vec<Vec<Codel>>,
) -> Option<(thread::JoinHandle<()>, sync::mpsc::Sender<(bool, Codel, Direction, Direction)>)> {
    let (width, height) = termion::terminal_size().unwrap();
    if (height as usize) <= picture.len() || (width as usize) < picture[0].len() {
        println!("Picture is larger than terminal size. View anyway [y/n]?");
        let mut input = String::new();
        if let Err(error) = io::stdin().read_line(&mut input) {
            panic!("{}", error);
        }
        if input.chars().next().unwrap() != 'y' {
            return None;
        }
    }
    let (se, re) = sync::mpsc::channel();
    let handle = thread::spawn(move || { display_pic(picture, re); });
    return Some((handle, se));
}
