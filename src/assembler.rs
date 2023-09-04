use std::{num::IntErrorKind, collections::{HashMap, VecDeque}};

use derive_new::new;

use crate::welt::{KarolWelt, Ziegel, Marke};

#[derive(Debug, Clone, Copy, new)]
struct Instr {
    op: usize,
    arg: isize,
}

fn text_to_nums(eingabe: &String) -> Vec<Instr> {
    //println!("{}", eingabe);

    let eingabe_iter = eingabe.split('\n');
    let mut ergebnis: Vec<Instr> = Vec::new();

    let mut labels: HashMap<&str, usize> = HashMap::new();
    let mut variables: HashMap<&str, usize> = HashMap::new();
    let mut i: usize = 0;
    for zeile in eingabe_iter.clone() {
        let aufgeteilt: Vec<&str> = zeile.trim().split(' ').collect();
        if vec!["hlt", "lda", "sta", "add", "neg", "jmp", "jnz", "ldad", "stad", "mka"].contains(&aufgeteilt[0]) {
            i += 1;
        } else
        if vec!["call", "ret"].contains(&aufgeteilt[0]) {
            i += 2;
        }

        if aufgeteilt[0] == "label" {
            labels.insert(aufgeteilt[1], i);
        }
        if aufgeteilt[0] == "var" {
            variables.insert(aufgeteilt[1], aufgeteilt[2].parse::<usize>().unwrap());
        }
    }
    println!("labels: {:?}", labels);
    println!("variables: {:?}", variables);

    for zeile in eingabe_iter {
        let aufgeteilt: Vec<&str> = zeile.trim().split(' ').collect();
        let mut n_instr = Instr::new(0, 0);

        match aufgeteilt[0] {
            "hlt" => {
                n_instr.op = 0;
                n_instr.arg = 0;
            },
            "lda" => {
                n_instr.op = 1;
                if variables.contains_key(&aufgeteilt[1]) {
                    n_instr.arg = variables[&aufgeteilt[1]] as isize;
                } else {
                    n_instr.arg = aufgeteilt[1].parse::<isize>().unwrap();
                }
            },
            "sta" => {
                n_instr.op = 2;
                if variables.contains_key(&aufgeteilt[1]) {
                    n_instr.arg = variables[&aufgeteilt[1]] as isize;
                } else {
                    n_instr.arg = aufgeteilt[1].parse::<isize>().unwrap();
                }
            },
            "add" => {
                n_instr.op = 3;
                if variables.contains_key(&aufgeteilt[1]) {
                    n_instr.arg = variables[&aufgeteilt[1]] as isize;
                } else {
                    n_instr.arg = aufgeteilt[1].parse::<isize>().unwrap();
                }
            },
            "neg" => {
                n_instr.op = 4;
                n_instr.arg = 0;
            },
            "jmp" => {
                n_instr.op = 5;
                if labels.contains_key(aufgeteilt[1]) {
                    n_instr.arg = labels[aufgeteilt[1]] as isize;
                } else {
                    n_instr.arg = aufgeteilt[1].parse::<isize>().unwrap();
                }
            },
            "jnz" => {
                n_instr.op = 6;
                if labels.contains_key(aufgeteilt[1]) {
                    n_instr.arg = labels[aufgeteilt[1]] as isize;
                } else {
                    n_instr.arg = aufgeteilt[1].parse::<isize>().unwrap();
                }
            },
            "ldad" => {
                n_instr.op = 7;
                if variables.contains_key(&aufgeteilt[1]) {
                    n_instr.arg = variables[&aufgeteilt[1]] as isize;
                } else {
                    n_instr.arg = aufgeteilt[1].parse::<isize>().unwrap();
                }
            },
            "stad" => {
                n_instr.op = 8;
                if variables.contains_key(&aufgeteilt[1]) {
                    n_instr.arg = variables[&aufgeteilt[1]] as isize;
                } else {
                    n_instr.arg = aufgeteilt[1].parse::<isize>().unwrap();
                }
            },
            "mka" => {
                n_instr.op = 9;
                n_instr.arg = aufgeteilt[1].parse::<isize>().unwrap();
            },
            "call" => {
                n_instr.op = 11;
                if labels.contains_key(aufgeteilt[1]) {
                    n_instr.arg = labels[aufgeteilt[1]] as isize;
                } else {
                    n_instr.arg = aufgeteilt[1].parse::<isize>().unwrap();
                }
            },
            "ret" => {
                n_instr.op = 12;
                n_instr.arg = 0;
            },

            _ => continue,
        }

        if n_instr.op > 10 {
            n_instr.op -= 10;
            ergebnis.push(Instr::new(10, 0));
            ergebnis.push(n_instr);
        } else {
            ergebnis.push(n_instr);
        }
    }

    println!("{:#?}", ergebnis);
    return ergebnis;
}

fn zahl_aufteilen(a: isize) -> Vec<usize> {
    let a: Vec<usize> = (a.abs() % 1000000).to_string().chars().map(|d| d.to_digit(10).unwrap() as usize).collect();
    let mut b: VecDeque<usize> = VecDeque::new();
    for elem in a {
        b.push_front(elem);
    }
    while b.len() < 6 {
        b.push_back(0);
    }
    return Vec::from(b);
}

fn nums_to_world(eingabe: Vec<Instr>) -> KarolWelt {
    let welt_breite = (eingabe.len() * 2 + 2).max(100);
    let mut welt = KarolWelt::new(welt_breite, 100, 10);
    //println!("breite: {}", welt_breite);

    for (i, instr) in eingabe.into_iter().enumerate() {
        let x = i * 2 + 1;
        for n in 0..instr.op {
            welt.feld[x][0].ziegel[n] = Ziegel::Rot;
        }
        welt.feld[x][8].marke = Marke::Blau;

        if instr.arg == 0 { continue; }

        if instr.arg < 0 {
            welt.feld[x + 1][1].marke = Marke::Grün;
        }

        for n in 0..6 {
            for a in 0..zahl_aufteilen(instr.arg)[n] {
                welt.feld[x + 1][n + 2].ziegel[a] = Ziegel::Rot;
            }
        }
    }

    return welt;
}

pub fn assemble(eingabe: &String) -> String {
    return nums_to_world(text_to_nums(eingabe)).to_string();
}