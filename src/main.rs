use std::{env, io};

const FILE: &'static str = "lib/input.csv";

mod file_utils;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        handle_mul_args(&args)
    } else {
        handle_no_args();
        Ok(())
    }
}

fn handle_mul_args(args: &Vec<String>) -> io::Result<()> {
    match args[1].trim() {
        "insert" => handle_insert(args)?,
        "remove" => handle_remove(args)?,
        _ => println!("Invalid usage."),
    }

    Ok(())
}

fn handle_remove(args: &Vec<String>) -> io::Result<()> {
    if args.len() != 3 {
        println!("Invalid usage.");
    }

    let k = &args[2];
    file_utils::remove_from_file(k, FILE)
}

fn handle_insert(args: &Vec<String>) -> io::Result<()> {
    if args.len() != 4 {
        println!("Invalid usage.");
    }

    let (k, v) = (args[2].clone(), args[3].parse::<u32>().unwrap());
    file_utils::insert_to_file(k, v, FILE)
}

fn handle_no_args() {
    let mut vec = file_utils::get_vec(FILE);
    calc_yap_costs(&mut vec);
    vec.sort_by(|a, b| b.2.total_cmp(&a.2));

    let n_len = get_longest_n(&vec);
    let c_len = get_longest_cc(&vec);
    let yc_len = get_longest_yc(&vec);
    println!("{yc_len}");

    for (i, (name, cc, yc)) in vec.iter().enumerate() {
        draw_box(
            n_len,
            c_len,
            yc_len,
            name,
            cc,
            yc,
            i == 0,
            i == vec.len() - 1,
        );
    }
}

fn draw_box(
    mut n_len: usize,
    mut c_len: usize,
    mut yc_len: usize,
    name: &str,
    cc: &u32,
    yc: &f32,
    start: bool,
    end: bool,
) {
    if n_len < 4 {
        n_len = 4;
    }

    if c_len < 5 {
        c_len = 5;
    }

    if yc_len < 8 {
        yc_len = 8;
    }

    // print start
    if start {
        println!(
            "┌NAME{}┬CHARS{}┬YAP─COST{}┐",
            (4..n_len).fold(String::new(), |acc, _| acc + "─"),
            (5..c_len).fold(String::new(), |acc, _| acc + "─"),
            (8..yc_len).fold(String::new(), |acc, _| acc + "─"),
        );
    }

    // print middle
    println!(
        "│{}{}│{}{}│{:.2}{}│",
        name,
        (name.len()..(n_len as usize)).fold(String::new(), |acc, _| acc + " "),
        cc,
        ((cc.ilog10() + 1)..(c_len as u32)).fold(String::new(), |acc, _| acc + " "),
        yc,
        (format!("{:.2}", yc).len()..yc_len).fold(String::new(), |acc, _| acc + " "),
    );

    // print end
    if end {
        println!(
            "└────{}┴─────{}┴────────{}┘",
            (4..n_len).fold(String::new(), |acc, _| acc + "─"),
            (5..c_len).fold(String::new(), |acc, _| acc + "─"),
            (8..yc_len).fold(String::new(), |acc, _| acc + "─"),
        );
    } else {
        println!(
            "├────{}┼─────{}┼────────{}┤",
            (4..n_len).fold(String::new(), |acc, _| acc + "─"),
            (5..c_len).fold(String::new(), |acc, _| acc + "─"),
            (8..yc_len).fold(String::new(), |acc, _| acc + "─"),
        );
    }
}

fn calc_yap_costs(vec: &mut Vec<(String, u32, f32)>) {
    let average = vec.iter().fold(0, |acc, (_, cc, _)| acc + cc) as f32 / vec.len() as f32;

    for (_, v, cost) in vec {
        if *v == 0 {
            *cost = 0.;
            continue;
        }
        *cost = *v as f32 / average as f32;
    }
}

fn get_longest_n(vec: &Vec<(String, u32, f32)>) -> usize {
    vec.iter().fold(0, |acc, (name, _, _)| {
        let len = name.len();
        if len > acc {
            len
        } else {
            acc
        }
    })
}

fn get_longest_cc(vec: &Vec<(String, u32, f32)>) -> usize {
    vec.iter().fold(0, |acc, (_, chars, _)| {
        let len = chars.ilog10() as usize + 1;
        if len > acc {
            len
        } else {
            acc
        }
    })
}

fn get_longest_yc(vec: &Vec<(String, u32, f32)>) -> usize {
    vec.iter().fold(0, |acc, (_, _, yc)| {
        let len = format!("{:.2}", yc).len();
        if len > acc {
            len
        } else {
            acc
        }
    })
}
