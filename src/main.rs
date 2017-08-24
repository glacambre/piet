#![feature(slice_patterns)]
#![feature(io)]

use std::io::prelude::*;

extern crate png;
extern crate getopts;

#[cfg(feature = "default")]
extern crate termion;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Lightness {
    Light,
    Normal,
    Dark,
}

impl Lightness {
    fn diff_to(self, other: &Lightness) -> usize {
        use Lightness::*;
        match self {
            Light => {
                match *other {
                    Light => 0,
                    Normal => 1,
                    Dark => 2,
                }
            },
            Normal => {
                match *other {
                    Light => 2,
                    Normal => 0,
                    Dark => 1,
                }
            },
            Dark => {
                match *other {
                    Light => 1,
                    Normal => 2,
                    Dark => 0,
                }
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Hue {
    Red,
    Yellow,
    Green,
    Cyan,
    Blue,
    Magenta,
    White,
    Black,
}

impl Hue {
    fn diff_to(self, other: &Hue) -> usize {
        use Hue::*;
        match self {
            Red => {
                match *other {
                    Red => 0,
                    Yellow => 1,
                    Green => 2,
                    Cyan => 3,
                    Blue => 4,
                    Magenta => 5,
                    White => 6,
                    Black => 6,
                }
            },
            Yellow => {
                match *other {
                    Red => 5,
                    Yellow => 0,
                    Green => 1,
                    Cyan => 2,
                    Blue => 3,
                    Magenta => 4,
                    White => 6,
                    Black => 6,
                }
            },
            Green => {
                match *other {
                    Red => 4,
                    Yellow => 5,
                    Green => 0,
                    Cyan => 1,
                    Blue => 2,
                    Magenta => 3,
                    White => 6,
                    Black => 6,
                }
            },
            Cyan => {
                match *other {
                    Red => 3,
                    Yellow => 4,
                    Green => 5,
                    Cyan => 0,
                    Blue => 1,
                    Magenta => 2,
                    White => 6,
                    Black => 6,
                }
            },
            Blue => {
                match *other {
                    Red => 2,
                    Yellow => 3,
                    Green => 4,
                    Cyan => 5,
                    Blue => 0,
                    Magenta => 1,
                    White => 6,
                    Black => 6,
                }
            },
            Magenta => {
                match *other {
                    Red => 1,
                    Yellow => 2,
                    Green => 3,
                    Cyan => 4,
                    Blue => 5,
                    Magenta => 0,
                    White => 6,
                    Black => 6,
                }
            },
            White => 6,
            Black => 6,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PietColor {
    lightness: Lightness,
    hue: Hue,
}

impl PietColor {
    fn diff_to(self, other: &PietColor) -> (usize, usize) {
        (self.hue.diff_to(&other.hue), self.lightness.diff_to(&other.lightness))
    }
}

#[cfg(feature = "default")]
impl std::fmt::Display for PietColor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Lightness::*;
        use termion::color::*;
        match self.hue {
            Hue::Red => {
                match self.lightness {
                    Light => write!(f, "{}{}LR", Bg(AnsiValue(217)), Fg(AnsiValue(217))),
                    Normal => write!(f, "{}{}NR", Bg(AnsiValue(196)), Fg(AnsiValue(196))),
                    Dark => write!(f, "{}{}DR", Bg(AnsiValue(124)), Fg(AnsiValue(124))),
                }
            },
            Hue::Yellow => {
                match self.lightness {
                    Light => write!(f, "{}{}LY", Bg(AnsiValue(229)), Fg(AnsiValue(229))),
                    Normal => write!(f, "{}{}NY", Bg(AnsiValue(226)), Fg(AnsiValue(226))),
                    Dark => write!(f, "{}{}DY", Bg(AnsiValue(142)), Fg(AnsiValue(142))),
                }
            },
            Hue::Green => {
                match self.lightness {
                    Light => write!(f, "{}{}LG", Bg(AnsiValue(157)), Fg(AnsiValue(157))),
                    Normal => write!(f, "{}{}NG", Bg(AnsiValue(46)), Fg(AnsiValue(46))),
                    Dark => write!(f, "{}{}DG", Bg(AnsiValue(34)), Fg(AnsiValue(34))),
                }
            },
            Hue::Cyan => {
                match self.lightness {
                    Light => write!(f, "{}{}LC", Bg(AnsiValue(159)), Fg(AnsiValue(159))),
                    Normal => write!(f, "{}{}NC", Bg(AnsiValue(51)), Fg(AnsiValue(51))),
                    Dark => write!(f, "{}{}DC", Bg(AnsiValue(37)), Fg(AnsiValue(37))),
                }
            },
            Hue::Blue => {
                match self.lightness {
                    Light => write!(f, "{}{}LB", Bg(AnsiValue(147)), Fg(AnsiValue(147))),
                    Normal => write!(f, "{}{}NB", Bg(AnsiValue(21)), Fg(AnsiValue(21))),
                    Dark => write!(f, "{}{}DB", Bg(AnsiValue(19)), Fg(AnsiValue(19))),
                }
            },
            Hue::Magenta => {
                match self.lightness {
                    Light => write!(f, "{}{}LM", Bg(AnsiValue(219)), Fg(AnsiValue(219))),
                    Normal => write!(f, "{}{}NM", Bg(AnsiValue(201)), Fg(AnsiValue(201))),
                    Dark => write!(f, "{}{}DM", Bg(AnsiValue(127)), Fg(AnsiValue(127))),
                }
            },
            Hue::White => write!(f, "{}{}WW", Bg(AnsiValue(231)), Fg(AnsiValue(231))),
            Hue::Black => write!(f, "{}{}DD", Bg(AnsiValue(0)), Fg(AnsiValue(0))),
        }
        // Hue::Red => {
        //     match self.lightness {
        //         Light => write!(f, "{} ", Bg(Rgb(255, 192, 192))),
        //         Normal => write!(f, "{} ", Bg(Rgb(255, 0, 0))),
        //         Dark => write!(f, "{} ", Bg(Rgb(192, 0, 0))),
        //     }
        // },
        // Hue::Yellow => {
        //     match self.lightness {
        //         Light => write!(f, "{} ", Bg(Rgb(255, 255, 192))),
        //         Normal => write!(f, "{} ", Bg(Rgb(255, 255, 0))),
        //         Dark => write!(f, "{} ", Bg(Rgb(192, 192, 0))),
        //     }
        // },
        // Hue::Green => {
        //     match self.lightness {
        //         Light => write!(f, "{} ", Bg(Rgb(192, 255, 192))),
        //         Normal => write!(f, "{} ", Bg(Rgb(0, 255, 0))),
        //         Dark => write!(f, "{} ", Bg(Rgb(0, 192, 0))),
        //     }
        // },
        // Hue::Cyan => {
        //     match self.lightness {
        //         Light => write!(f, "{} ", Bg(Rgb(192, 255, 255))),
        //         Normal => write!(f, "{} ", Bg(Rgb(0, 255, 255))),
        //         Dark => write!(f, "{} ", Bg(Rgb(255, 192, 192))),
        //     }
        // },
        // Hue::Blue => {
        //     match self.lightness {
        //         Light => write!(f, "{} ", Bg(Rgb(192, 192, 255))),
        //         Normal => write!(f, "{} ", Bg(Rgb(0, 0, 255))),
        //         Dark => write!(f, "{} ", Bg(Rgb(0, 0, 192))),
        //     }
        // },
        // Hue::Magenta => {
        //     match self.lightness {
        //         Light => write!(f, "{} ", Bg(Rgb(255, 192, 255))),
        //         Normal => write!(f, "{} ", Bg(Rgb(255, 0, 255))),
        //         Dark => write!(f, "{} ", Bg(Rgb(192, 0, 192))),
        //     }
        // },
        // Hue::White => write!(f, "{} ", Bg(Rgb(255, 255, 255))),
        // Hue::Black => write!(f, "{} ", Bg(Rgb(0, 0, 0))),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Codel {
    color: PietColor,
    x: usize,
    y: usize,
}

impl Codel {
    fn diff_to(self, other: &Codel) -> (usize, usize) {
        self.color.diff_to(&other.color)
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Left,
    Down,
    Up,
}

impl Direction {
    fn to_vector(self) -> (isize, isize) {
        match self {
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
            Direction::Down => (0, 1),
            Direction::Up => (0, -1),
        }
    }

    fn rotate(self) -> Direction {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }

    fn opposite(self) -> Direction {
        match self {
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Up => Direction::Down,
        }
    }
}

/* Uses the direction pointer and the codel chooser in order to decide which of the two Codels is
 * the farthest. */
// First, use the direction pointer. Check if a Codel is farther than the other and return it in
// this case. If it's a tie, use the codel chooser in order to decide which codel should be chosen.
//
fn compare_farthest_codels<'a>(
    c1: &'a Codel,
    c2: &'a Codel,
    dp: Direction,
    cc: Direction,
) -> &'a Codel {
    use Direction::*;
    if c1 == c2 {
        c1
    } else {
        match dp {
            Right => {
                if c1.x > c2.x {
                    &c1
                } else if c1.x < c2.x {
                    &c2
                } else {
                    match cc {
                        Left => if c1.y < c2.y { c1 } else { c2 },
                        Right => if c1.y > c2.y { c1 } else { c2 },
                        _ => panic!("Codel chooser is neither Left or Right"),
                    }
                }
            },
            Left => {
                if c1.x < c2.x {
                    &c1
                } else if c1.x > c2.x {
                    &c2
                } else {
                    match cc {
                        Left => if c1.y > c2.y { c1 } else { c2 },
                        Right => if c1.y < c2.y { c1 } else { c2 },
                        _ => panic!("Codel chooser is neither Left or Right"),
                    }
                }
            },
            Up => {
                if c1.y < c2.y {
                    &c1
                } else if c1.y > c2.y {
                    &c2
                } else {
                    match cc {
                        Left => if c1.x < c2.x { c1 } else { c2 },
                        Right => if c1.x > c2.x { c1 } else { c2 },
                        _ => panic!("Codel chooser is neither Left or Right"),
                    }
                }
            },
            Down => {
                if c1.y > c2.y {
                    &c1
                } else if c1.y < c2.y {
                    &c2
                } else {
                    match cc {
                        Left => if c1.x > c2.x { c1 } else { c2 },
                        Right => if c1.x < c2.x { c1 } else { c2 },
                        _ => panic!("Codel chooser is neither Left or Right"),
                    }
                }
            },
        }
    }
}

/* Fetches the farthest codel belonging to the same block as the codel at position x, y
 * picture: A matrix of Codels corresponding to the piet program.
 * x: The starting position within the matrix on the x axis
 * y: The starting position within the matriy on the y ayis
 * dp: The global direction pointer
 * cc: The global codel chooser
 * returns: An (usize, &Codel) tuple. For non-white blocks, the usize corresponds to the size of
 * the block, the &Codel is the farthest codel that belongs to the same block as the starting
 * position according to the direction pointer and cc.
 * For white blocks the usize is 0 and the codel is the farthest white codel in a straight line
 * in
 * dp's direction. */
// This is basically a depth-first search implemented using a stack instead of recursion.
fn get_farthest_codel(
    picture: &Vec<Vec<Codel>>,
    x: usize,
    y: usize,
    dp: Direction,
    cc: Direction,
) -> (usize, &Codel) {
    // White blocks have a different
    if picture[y][x].color.hue == Hue::White {
        let (mut x, mut y) = (x as isize, y as isize);
        let (tmpx, tmpy) = dp.to_vector();
        while y >= 0 && y < picture.len() as isize && x >= 0 &&
            x < picture[y as usize].len() as isize &&
            picture[y as usize][x as usize].color.hue == Hue::White
        {
            y += tmpy;
            x += tmpx;
        }
        y -= tmpy;
        x -= tmpx;
        return (0, &picture[y as usize][x as usize]);
    }

    let mut codels_to_visit = vec![(x, y); 1];
    let mut visited_codels = std::collections::HashSet::new();

    let mut result = &picture[y][x];
    while codels_to_visit.len() > 0 {
        // Get the last codel added to the codel stack
        let (tmpx, tmpy) = codels_to_visit.pop().unwrap();
        // Mark it as visited
        visited_codels.insert(&picture[tmpy][tmpx]);
        // If the codel under the current codel is within our matrix bounds, its color matches the
        // starting color and it hasn't been visited yet
        if tmpy + 1 < picture.len() && &picture[tmpy + 1][tmpx].color == &picture[y][x].color &&
            !visited_codels.contains(&picture[tmpy + 1][tmpx])
        {
            // Add it to the list of codels that need to be visited
            codels_to_visit.push((tmpx, tmpy + 1));
        }
        // If the codel above the current codel is within the bounds, the right colors and hasn't
        // been visited
        if tmpy > 0 && &picture[tmpy - 1][tmpx].color == &picture[y][x].color &&
            !visited_codels.contains(&picture[tmpy - 1][tmpx])
        {
            // Add it to the list of codels that need to be visited
            codels_to_visit.push((tmpx, tmpy - 1));
        }
        // Same thing for the codel on the right
        if tmpx + 1 < picture[tmpy].len() &&
            &picture[tmpy][tmpx + 1].color == &picture[y][x].color &&
            !visited_codels.contains(&picture[tmpy][tmpx + 1])
        {
            codels_to_visit.push((tmpx + 1, tmpy));
        }
        // Same thing for the codel on the left
        if tmpx > 0 && &picture[tmpy][tmpx - 1].color == &picture[y][x].color &&
            !visited_codels.contains(&picture[tmpy][tmpx - 1])
        {
            codels_to_visit.push((tmpx - 1, tmpy));
        }
        result = compare_farthest_codels(result, &picture[tmpy][tmpx], dp, cc);
    }

    return (visited_codels.len(), result);
}

/* Checks whether there is a non-white, non-black codel aligned with the current Codel in the
 * direction of the dp within the picture.
 * returns: A (bool, bool, &Codel) tuple. The first boolean is true if there is an available
 * codel
 * in the direction of the direction pointer, false otherwise. The second bool indicates whether
 * the algorithm encountered white blocks between the current codel and the newly found codel.
 * The
 * returned Codel is the new codel when the bool is true and the starting codel when the bool is
 * false.
 * */
fn can_go_in_direction<'a>(
    picture: &'a Vec<Vec<Codel>>,
    current_codel: &'a Codel,
    dp: Direction,
) -> (bool, &'a Codel) {
    let (mut tmpx, mut tmpy) = (current_codel.x as isize, current_codel.y as isize);
    let vec = dp.to_vector();
    tmpx += vec.0;
    tmpy += vec.1;

    // If we're not within the matrix's bounds or if the codel is black, we can't go in this
    // direction
    if tmpy < 0 || tmpy as usize >= picture.len() || tmpx < 0 ||
        (tmpx as usize) >= picture[(tmpy as usize)].len() ||
        picture[tmpy as usize][tmpx as usize].color.hue == Hue::Black
    {
        return (false, current_codel);
    }
    // Return the found codel
    return (true, &picture[tmpy as usize][tmpx as usize]);
}

fn get_picture(
    filename: &std::string::String,
    codel_size: usize,
    default_color: PietColor,
) -> Vec<Vec<Codel>> {
    let decoder = png::Decoder::new(std::fs::File::open(filename).unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();
    let mut buffer = vec![0; info.buffer_size()];
    reader.next_frame(&mut buffer).unwrap();

    let values_per_pixel;
    match info.color_type {
        png::ColorType::RGBA => values_per_pixel = 4,
        png::ColorType::RGB => values_per_pixel = 3,
        _ => panic!("PNG picture is neither RGB or RGBA"),
    }
    let pic_width = info.width as usize;
    let pic_height = info.height as usize;

    let mut picture: Vec<Vec<Codel>> =
        vec![
            vec![Codel { color: default_color.clone(), x: 0, y: 0 }; pic_width / codel_size];
            pic_height / codel_size
        ];
    let mut i = -1;
    for pixel in buffer.chunks(values_per_pixel) {
        i += 1;
        let (mut x, mut y) = ((i as usize) % pic_width, (i as usize) / pic_width);
        if x % codel_size != 0 || y % codel_size != 0 {
            continue;
        }
        x = x / codel_size;
        y = y / codel_size;
        picture[y][x].x = x;
        picture[y][x].y = y;
        picture[y][x].color = match &pixel[0..3] {
            &[255, 192, 192] => PietColor { hue: Hue::Red, lightness: Lightness::Light },
            &[255, 0, 0] => PietColor { hue: Hue::Red, lightness: Lightness::Normal },
            &[192, 0, 0] => PietColor { hue: Hue::Red, lightness: Lightness::Dark },
            &[255, 255, 192] => PietColor { hue: Hue::Yellow, lightness: Lightness::Light },
            &[255, 255, 0] => PietColor { hue: Hue::Yellow, lightness: Lightness::Normal },
            &[192, 192, 0] => PietColor { hue: Hue::Yellow, lightness: Lightness::Dark },
            &[192, 255, 192] => PietColor { hue: Hue::Green, lightness: Lightness::Light },
            &[0, 255, 0] => PietColor { hue: Hue::Green, lightness: Lightness::Normal },
            &[0, 192, 0] => PietColor { hue: Hue::Green, lightness: Lightness::Dark },
            &[192, 255, 255] => PietColor { hue: Hue::Cyan, lightness: Lightness::Light },
            &[0, 255, 255] => PietColor { hue: Hue::Cyan, lightness: Lightness::Normal },
            &[0, 192, 192] => PietColor { hue: Hue::Cyan, lightness: Lightness::Dark },
            &[192, 192, 255] => PietColor { hue: Hue::Blue, lightness: Lightness::Light },
            &[0, 0, 255] => PietColor { hue: Hue::Blue, lightness: Lightness::Normal },
            &[0, 0, 192] => PietColor { hue: Hue::Blue, lightness: Lightness::Dark },
            &[255, 192, 255] => PietColor { hue: Hue::Magenta, lightness: Lightness::Light },
            &[255, 0, 255] => PietColor { hue: Hue::Magenta, lightness: Lightness::Normal },
            &[192, 0, 192] => PietColor { hue: Hue::Magenta, lightness: Lightness::Dark },
            &[0, 0, 0] => PietColor { hue: Hue::Black, lightness: Lightness::Normal },
            &[255, 255, 255] => PietColor { hue: Hue::White, lightness: Lightness::Normal },
            _ => default_color.clone(),
        };
    }
    return picture;
}

#[cfg(feature = "default")]
fn display_pic(picture: &Vec<Vec<Codel>>, current_codel: &Codel, dp: Direction, cc: Direction) {
    for row in picture.iter() {
        for codel in row.iter() {
            print!("{}", codel.color);
        }
        println!("{}", termion::style::Reset);
    }
    print!("{:?}{:?}{:?}", current_codel, dp, cc);
}

fn main() {
    use getopts::Options;
    use std::env;

    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("b", "black", "Use black as default color instead of white.");
    opts.optopt("c", "codel_size", "Number of pixels per codels. Default: 1", "2");
    opts.optflag("d", "debug", "Use debug mode.");
    #[cfg(feature = "default")] { 
        opts.optflag("v", "view", "Display the program being run.");
    }
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => { print!("{}{}", e.to_string(), opts.usage("")); std::process::exit(1) },
    };

    if matches.free.len() != 1 {
        let brief = format!("Usage: {} file.png [options]", args[0]);
        print!("{}", opts.usage(&brief));
        std::process::exit(1);
    }

    let debug = matches.opt_present("d");
    let mut codel_size = 1;
    if matches.opt_present("c") {
        if let Ok(num) = matches.opt_str("c").unwrap().parse::<usize>() {
            codel_size = if num > 0 { num } else { 1 };
        } else {
            println!("Error: codel size has to be a greater than 0.");
            std::process::exit(1);
        }
    }

    let default_color = if matches.opt_present("b") {
        PietColor { hue: Hue::Black, lightness: Lightness::Normal }
    } else {
        PietColor { hue: Hue::White, lightness: Lightness::Normal }
    };

    let picture = get_picture(&matches.free[0], codel_size, default_color);
    let mut current_codel = picture[0][0].clone();
    let mut piet_stack: Vec<isize> = Vec::new();
    let mut dp = Direction::Right;
    let mut cc = Direction::Left;
    let mut chars = std::io::stdin().chars();

    #[cfg(feature = "default")]
    let mut view_program = matches.opt_present("v");

    #[cfg(feature = "default")] {
        if view_program {
            let (width, height) = termion::terminal_size().unwrap();
            if (height as usize) <= picture.len() || (width as usize) < picture[0].len() {
                println!("Picture is larger than terminal size. View anyway [y/n]?");
                let mut input = String::new();
                match std::io::stdin().read_line(&mut input) {
                    Ok(_) => view_program = input.chars().next().unwrap() == 'y',
                    Err(error) => panic!(error),
                }
            }
        }
    }

    'main_loop: loop {
        #[cfg(feature = "default")] { 
            if view_program {
                display_pic(&picture, &current_codel, dp, cc);
                println!("{:?}", piet_stack);
            }
        }
        if debug {
            println!("{:?}, {:?}, {:?}", current_codel, dp, cc);
            println!("{:?}", piet_stack);
        }
        let next_codel;
        let mut attempts = 0;
        let mut block_size;
        'codel_choosing: loop {
            let result = get_farthest_codel(&picture, current_codel.x, current_codel.y, dp, cc);
            block_size = result.0;
            let farthest_codel = result.1;
            match (attempts % 2, can_go_in_direction(&picture, farthest_codel, dp)) {
                (_, (true, codel)) => {
                    next_codel = codel;
                    break 'codel_choosing;
                },
                (0, (false, codel)) => {
                    current_codel = codel.clone();
                    cc = cc.opposite();
                },
                (1, (false, codel)) => {
                    current_codel = codel.clone();
                    dp = dp.rotate();
                },
                _ => unreachable!(),
            }
            attempts += 1;
            if attempts >= 8 {
                break 'main_loop;
            }
        }
        match current_codel.diff_to(next_codel) {
            // Push
            (0, 1) => piet_stack.push(block_size as isize),
            // Pop
            (0, 2) => {
                piet_stack.pop();
            },
            // Add
            (1, 0) => {
                if let (Some(a), Some(b)) = (piet_stack.pop(), piet_stack.pop()) {
                    piet_stack.push(b + a);
                }
            },
            // Sub
            (1, 1) => {
                if let (Some(a), Some(b)) = (piet_stack.pop(), piet_stack.pop()) {
                    piet_stack.push(b - a);
                }
            },
            // Mul
            (1, 2) => {
                if let (Some(a), Some(b)) = (piet_stack.pop(), piet_stack.pop()) {
                    piet_stack.push(b * a);
                }
            },
            // Div
            (2, 0) => {
                if let (Some(a), Some(b)) = (piet_stack.pop(), piet_stack.pop()) {
                    piet_stack.push(b / a);
                }
            },
            // Mod
            (2, 1) => {
                if let (Some(a), Some(b)) = (piet_stack.pop(), piet_stack.pop()) {
                    piet_stack.push((b % a) + a);
                }
            },
            // Not
            (2, 2) => {
                if let Some(val) = piet_stack.pop() {
                    piet_stack.push(if val == 0 { 1 } else { 0 })
                }
            },
            // Greater
            (3, 0) => {
                if let (Some(a), Some(b)) = (piet_stack.pop(), piet_stack.pop()) {
                    piet_stack.push(if b > a { 1 } else { 0 })
                }
            },
            // Pointer
            (3, 1) => {
                if let Some(a) = piet_stack.pop() {
                    for _ in 0..a {
                        dp = dp.rotate();
                    }
                }
            },
            // Switch
            (3, 2) => {
                if let Some(a) = piet_stack.pop() {
                    for _ in 0..a {
                        cc = cc.opposite();
                    }
                }
            },
            // Duplicate
            (4, 0) => {
                if let Some(a) = piet_stack.pop() {
                    piet_stack.push(a);
                    piet_stack.push(a);
                }
            },
            // Roll
            (4, 1) => {
                if let (Some(count), Some(depth)) = (piet_stack.pop(), piet_stack.pop()) {
                    let mut before: Vec<_> = piet_stack
                        .iter()
                        .take(piet_stack.len() - (depth as usize))
                        .map(|e| *e)
                        .collect();
                    if count >= 0 {
                        let mut rolled: Vec<_> = piet_stack
                            .iter()
                            .skip(piet_stack.len() - (count as usize))
                            .map(|e| *e)
                            .collect();
                        let mut after: Vec<_> = piet_stack
                            .iter()
                            .skip(piet_stack.len() - (depth as usize))
                            .take((depth - count) as usize)
                            .map(|e| *e)
                            .collect();
                        before.append(&mut rolled);
                        before.append(&mut after);
                    } else {
                        let mut rolled: Vec<_> = piet_stack
                            .iter()
                            .skip(before.len())
                            .take(count.abs() as usize)
                            .map(|e| *e)
                            .collect();
                        let mut after: Vec<_> = piet_stack
                            .iter()
                            .skip(before.len() + rolled.len())
                            .map(|e| *e)
                            .collect();
                        before.append(&mut after);
                        while let Some(v) = rolled.pop() {
                            before.push(v);
                        }
                    }
                    piet_stack = before;
                }
            },
            // in(number)
            (4, 2) => {
                let mut input = String::new();
                match std::io::stdin().read_line(&mut input) {
                    Ok(_) => piet_stack.push(input.trim_right().parse().unwrap()),
                    Err(error) => {
                        if debug {
                            panic!(error)
                        }
                    },
                }
            },
            // in(char)
            (5, 0) => {
                if let Some(Ok(char)) = chars.next() {
                    piet_stack.push(char as isize);
                } else {
                    chars = std::io::stdin().chars();
                    if let Some(Ok(char)) = chars.next() {
                        piet_stack.push(char as isize);
                    }
                }
            },
            // out(number)
            (5, 1) => {
                if let Some(val) = piet_stack.pop() {
                    print!("{}", val)
                }
            },
            // out(char)
            (5, 2) => {
                if let Some(val) = piet_stack.pop() {
                    print!("{}", std::char::from_u32(val as u32).unwrap())
                }
            },
            (6, _) => (),
            (a, b) => panic!("Error: differences are ({},{}))", a, b),
        }
        current_codel = next_codel.clone();
    }
}
