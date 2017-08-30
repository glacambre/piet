#![feature(slice_patterns)]
#![feature(io)]
#![feature(asm)]

mod pietcolor;
mod codel;

use pietcolor::*;
use codel::*;
use std::*;

use std::io::prelude::*;

extern crate png;
extern crate getopts;

#[cfg(feature = "default")]
use display::*;
#[cfg(feature = "default")]
mod display;
#[cfg(feature = "default")]
extern crate termion;

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
 * in dp's direction.  For other blocks, this is basically a depth-first search implemented using
 * a stack instead of recursion.
 * */
fn get_farthest_codel(
    picture: &Vec<Vec<Codel>>,
    x: usize,
    y: usize,
    dp: Direction,
    cc: Direction,
) -> (usize, &Codel) {
    // White blocks have a different algorithm
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
    let mut visited_codels = collections::HashSet::new();

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
        // Same thing
        if tmpy > 0 && &picture[tmpy - 1][tmpx].color == &picture[y][x].color &&
            !visited_codels.contains(&picture[tmpy - 1][tmpx])
        {
            codels_to_visit.push((tmpx, tmpy - 1));
        }
        // Same thing
        if tmpx + 1 < picture[tmpy].len() &&
            &picture[tmpy][tmpx + 1].color == &picture[y][x].color &&
            !visited_codels.contains(&picture[tmpy][tmpx + 1])
        {
            codels_to_visit.push((tmpx + 1, tmpy));
        }
        // Same thing
        if tmpx > 0 && &picture[tmpy][tmpx - 1].color == &picture[y][x].color &&
            !visited_codels.contains(&picture[tmpy][tmpx - 1])
        {
            codels_to_visit.push((tmpx - 1, tmpy));
        }
        result = result.compare_to(&picture[tmpy][tmpx], dp, cc);
    }

    return (visited_codels.len(), result);
}

/* Checks whether there is a non-white, non-black codel aligned with the current Codel in the
 * direction of the dp within the picture.
 * returns: A (bool, bool, &Codel) tuple. The first boolean is true if there is an available
 * codel in the direction of the direction pointer, false otherwise. The second bool indicates
 * whether the algorithm encountered white blocks between the current codel and the newly found
 * codel.  The returned Codel is the new codel when the bool is true and the starting codel when
 * the bool is false.
 * */
fn can_go_in_direction<'a>(
    picture: &'a Vec<Vec<Codel>>,
    cur_codel: &'a Codel,
    dp: Direction,
) -> (bool, &'a Codel) {
    let (mut tmpx, mut tmpy) = (cur_codel.x as isize, cur_codel.y as isize);
    let vec = dp.to_vector();
    tmpx += vec.0;
    tmpy += vec.1;

    // If we're not within the matrix's bounds or if the codel is black, we can't go in this
    // direction
    if tmpy < 0 || tmpy as usize >= picture.len() || tmpx < 0 ||
        (tmpx as usize) >= picture[(tmpy as usize)].len() ||
        picture[tmpy as usize][tmpx as usize].color.hue == Hue::Black
    {
        return (false, cur_codel);
    }
    // Return the found codel
    return (true, &picture[tmpy as usize][tmpx as usize]);
}

fn get_picture(
    filename: &string::String,
    codel_size: usize,
    default_color: PietColor,
    syscalls_enabled: bool,
) -> Vec<Vec<Codel>> {
    let decoder = png::Decoder::new(fs::File::open(filename).unwrap());
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
    let mut i: isize = -1;
    let syscall_codel = if syscalls_enabled { PietColor { hue: Hue::Smoke, lightness: Lightness::Normal } } else { default_color.clone() };
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
            &[0xFF, 0xC0, 0xC0] => PietColor { hue: Hue::Red, lightness: Lightness::Light },
            &[0xFF, 0x00, 0x00] => PietColor { hue: Hue::Red, lightness: Lightness::Normal },
            &[0xC0, 0x00, 0x00] => PietColor { hue: Hue::Red, lightness: Lightness::Dark },
            &[0xFF, 0xFF, 0xC0] => PietColor { hue: Hue::Yellow, lightness: Lightness::Light },
            &[0xFF, 0xFF, 0x00] => PietColor { hue: Hue::Yellow, lightness: Lightness::Normal },
            &[0xC0, 0xC0, 0x00] => PietColor { hue: Hue::Yellow, lightness: Lightness::Dark },
            &[0xC0, 0xFF, 0xC0] => PietColor { hue: Hue::Green, lightness: Lightness::Light },
            &[0x00, 0xFF, 0x00] => PietColor { hue: Hue::Green, lightness: Lightness::Normal },
            &[0x00, 0xC0, 0x00] => PietColor { hue: Hue::Green, lightness: Lightness::Dark },
            &[0xC0, 0xFF, 0xFF] => PietColor { hue: Hue::Cyan, lightness: Lightness::Light },
            &[0x00, 0xFF, 0xFF] => PietColor { hue: Hue::Cyan, lightness: Lightness::Normal },
            &[0x00, 0xC0, 0xC0] => PietColor { hue: Hue::Cyan, lightness: Lightness::Dark },
            &[0xC0, 0xC0, 0xFF] => PietColor { hue: Hue::Blue, lightness: Lightness::Light },
            &[0x00, 0x00, 0xFF] => PietColor { hue: Hue::Blue, lightness: Lightness::Normal },
            &[0x00, 0x00, 0xC0] => PietColor { hue: Hue::Blue, lightness: Lightness::Dark },
            &[0xFF, 0xC0, 0xFF] => PietColor { hue: Hue::Magenta, lightness: Lightness::Light },
            &[0xFF, 0x00, 0xFF] => PietColor { hue: Hue::Magenta, lightness: Lightness::Normal },
            &[0xC0, 0x00, 0xC0] => PietColor { hue: Hue::Magenta, lightness: Lightness::Dark },
            &[0x00, 0x00, 0x00] => PietColor { hue: Hue::Black, lightness: Lightness::Normal },
            &[0xFF, 0xFF, 0xFF] => PietColor { hue: Hue::White, lightness: Lightness::Normal },
            &[0xC0, 0xC0, 0xC0] => syscall_codel.clone(),
            _ => default_color.clone(),
        };
    }
    return picture;
}

fn main() {
    use getopts::Options;
    use env;

    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("b", "black", "Use black as default color instead of white.");
    opts.optopt("c", "codel_size", "Number of pixels per codels. Default: 1", "2");
    opts.optflag("d", "debug", "Use debug mode.");
    opts.optflag("s", "syscalls", "Enable the syscall instruction (color #C0C0C0)");
    #[cfg(feature = "default")]
    {
        opts.optflag("v", "view", "Display the program being run.");
    }
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            print!("{}{}", e.to_string(), opts.usage(""));
            process::exit(1)
        },
    };

    if matches.free.len() != 1 {
        let brief = format!("Usage: {} file.png [options]", args[0]);
        print!("{}", opts.usage(&brief));
        process::exit(1);
    }

    let debug = matches.opt_present("d");
    let mut codel_size = 1;
    if matches.opt_present("c") {
        if let Ok(num) = matches.opt_str("c").unwrap().parse::<usize>() {
            codel_size = if num > 0 { num } else { 1 };
        } else {
            println!("Error: codel size has to be a greater than 0.");
            process::exit(1);
        }
    }

    let default_color = if matches.opt_present("b") {
        PietColor { hue: Hue::Black, lightness: Lightness::Normal }
    } else {
        PietColor { hue: Hue::White, lightness: Lightness::Normal }
    };

    let picture = get_picture(&matches.free[0], codel_size, default_color, matches.opt_present("s"));
    let mut cur_codel = picture[0][0].clone();
    let mut piet_stack: Vec<i64> = Vec::new();
    let mut dp = Direction::Right;
    let mut cc = Direction::Left;
    let mut chars = io::stdin().chars();

    #[cfg(feature = "default")]
    let disp_thread = if matches.opt_present("v") { setup_display(picture.clone()) } else { None };

    'main_loop: loop {
        #[cfg(feature = "default")]
        {
            if let Some((_, ref channel)) = disp_thread {
                if let Err(e) = channel.send((true, cur_codel.clone(), dp.clone(), cc.clone())) {
                    println!("{}", e);
                }
            }
        }
        if debug {
            println!("{:?}, {:?}, {:?}", cur_codel, dp, cc);
            println!("{:?}", piet_stack);
            if piet_stack.len() > 1 {
            println!("{:>064b}", piet_stack[0]);
            }
        }
        let next_codel;
        let mut attempts = 0;
        let mut block_size;
        'codel_choosing: loop {
            let result = get_farthest_codel(&picture, cur_codel.x, cur_codel.y, dp, cc);
            block_size = result.0;
            let farthest_codel = result.1;
            match (attempts % 2, can_go_in_direction(&picture, farthest_codel, dp)) {
                (_, (true, codel)) => {
                    next_codel = codel;
                    break 'codel_choosing;
                },
                (0, (false, codel)) => {
                    cur_codel = codel.clone();
                    cc = cc.opposite();
                },
                (1, (false, codel)) => {
                    cur_codel = codel.clone();
                    dp = dp.rotate();
                },
                _ => unreachable!(),
            }
            attempts += 1;
            if attempts >= 8 {
                break 'main_loop;
            }
        }
        match cur_codel.diff_to(next_codel) {
            // Push
            (0, 1) => piet_stack.push(block_size as i64),
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
                match io::stdin().read_line(&mut input) {
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
                    piet_stack.push(char as i64);
                } else {
                    chars = io::stdin().chars();
                    if let Some(Ok(char)) = chars.next() {
                        piet_stack.push(char as i64);
                    }
                }
            },
            // out(number)
            (5, 1) => {
                if let Some(val) = piet_stack.pop() {
                    #[cfg(feature = "default")]
                    {
                        if let Some(_) = disp_thread {
                            print!(
                                "{}{}{}",
                                termion::screen::ToMainScreen,
                                val,
                                termion::screen::ToAlternateScreen
                            );
                        }
                    }

                    print!("{}", val);
                }
            },
            // out(char)
            (5, 2) => {
                if let Some(val) = piet_stack.pop() {
                    if let Some(val) = char::from_u32(val as u32) {
                        #[cfg(feature = "default")]
                        {
                            if let Some(_) = disp_thread {
                                print!(
                                    "{}{}{}",
                                    termion::screen::ToMainScreen,
                                    val,
                                    termion::screen::ToAlternateScreen
                                );
                            }
                        }
                        print!("{}", val);
                    }
                }
            },
            (6, _) => {
                if cur_codel.color.hue == Hue::Smoke {
                    println!("syscall!");
                    if let Some(syscall_num) = piet_stack.pop() {
                        let mut syscall_num = syscall_num;
                        let mut syscall_args = vec![];
                        if let Some(arg_count) = piet_stack.pop() {
                            let stack_addr = &piet_stack[0] as *const i64 as u64;
                            for _ in 0..arg_count {
                                if let (Some(arg_type), Some(arg)) = (piet_stack.pop(), piet_stack.pop()) {
                                    match arg_type {
                                        1 => syscall_args.push(arg),
                                        2 => syscall_args.push((if arg < 0 { (stack_addr - arg.abs() as u64) as i64 } else { (stack_addr + arg as u64) as i64 })),
                                        _ => println!("Bad arg_type!"),
                                    }
                                }
                            }
                        }
                        unsafe {
                        let result = match syscall_args.len() {
                            0 => { asm!("syscall"
                                      : "+{rax}"(syscall_num)
                                      :
                                      : "rcx", "r11", "memory"
                                      : "volatile"); syscall_num },
                            1 => { asm!("syscall"
                                      : "+{rax}"(syscall_num)
                                      : "{rdi}"(syscall_args[0])
                                      : "rcx", "r11", "memory"
                                      : "volatile"); syscall_num },
                            2 => { asm!("syscall"
                                      : "+{rax}"(syscall_num)
                                      : "{rdi}"(syscall_args[0]) "{rsi}"(syscall_args[1])
                                      : "rcx", "r11", "memory"
                                      : "volatile"); syscall_num },
                            3 => { asm!("syscall"
                                      : "+{rax}"(syscall_num)
                                      : "{rdi}"(syscall_args[0]) "{rsi}"(syscall_args[1]) "{rdx}"(syscall_args[2])
                                      : "rcx", "r11", "memory"
                                      : "volatile"); syscall_num },
                            4 => { asm!("syscall"
                                      : "+{rax}"(syscall_num)
                                      : "{rdi}"(syscall_args[0]) "{rsi}"(syscall_args[1]) "{rdx}"(syscall_args[2]) "{r10}"(syscall_args[3])
                                      : "rcx", "r11", "memory"
                                      : "volatile"); syscall_num },
                            5 => { asm!("syscall"
                                      : "+{rax}"(syscall_num)
                                      : "{rdi}"(syscall_args[0]) "{rsi}"(syscall_args[1]) "{rdx}"(syscall_args[2]) "{r10}"(syscall_args[3]) "{r8}"(syscall_args[4])
                                      : "rcx", "r11", "memory"
                                      : "volatile"); syscall_num },
                            _ => { asm!("syscall"
                                      : "+{rax}"(syscall_num)
                                      : "{rdi}"(syscall_args[0]) "{rsi}"(syscall_args[1]) "{rdx}"(syscall_args[2]) "{r10}"(syscall_args[3]) "{r8}"(syscall_args[4])"{r9}"(syscall_args[5])
                                      : "rcx", "r11", "memory"
                                      : "volatile"); syscall_num },
                        };
                        piet_stack.push(result);
                        }
                    }
                }
            },
            (a, b) => panic!("Error: differences are ({},{}))", a, b),
        }
        cur_codel = next_codel.clone();
    }

    #[cfg(feature = "default")]
    {
        if let Some((handle, channel)) = disp_thread {
            if let Err(e) = channel.send((false, cur_codel, dp, cc)) {
                println!("{}", e);
            }
            if let Err(e) = handle.join() {
                println!("{:?}", e);
            }
        }
    }
}
