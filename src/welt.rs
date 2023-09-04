#![allow(dead_code)]

const VERSION_STRING: &str = "KarolVersion3.0";

#[derive(Debug, Clone, Copy)]
pub enum Ziegel {
    Leer = 'n' as isize,
    Rot  = 'A' as isize,
    Gelb = 'B' as isize,
    Blau = 'C' as isize,
    Grün = 'D' as isize,
    Quader = 'q' as isize,
}

impl Ziegel {
    fn from_char(c: char) -> Option<Ziegel> {
        match c {
            'n' => Some(Ziegel::Leer),
            'A' => Some(Ziegel::Rot),
            'B' => Some(Ziegel::Gelb),
            'C' => Some(Ziegel::Blau),
            'D' => Some(Ziegel::Grün),
            'q' => Some(Ziegel::Quader),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Marke {
    Leer = 'o' as isize,
    Rot  = 'K' as isize,
    Gelb = 'L' as isize,
    Blau = 'M' as isize,
    Grün = 'N' as isize,
}

impl Marke {
    fn from_char(c: char) -> Option<Marke> {
        match c {
            'o' => Some(Marke::Leer),
            'K' => Some(Marke::Rot),
            'L' => Some(Marke::Gelb),
            'M' => Some(Marke::Blau),
            'N' => Some(Marke::Grün),
            _ => None,
        }
    }
}

pub struct WeltPosition {
    pub marke: Marke,
    pub ziegel: Vec<Ziegel>,
}

impl WeltPosition {
    pub fn new(hoehe: usize) -> WeltPosition {
        let mut tmp: Vec<Ziegel> = Vec::new();
        for _ in 0..hoehe { tmp.push(Ziegel::Leer) }
        return WeltPosition { marke: Marke::Leer, ziegel: tmp }
    }
}

pub struct KarolWelt {
    breite: usize,
    laenge: usize,
    hoehe: usize,
    pub feld: Vec<Vec<WeltPosition>>,
}

impl KarolWelt {
    pub fn new(breite: usize, laenge: usize, hoehe: usize) -> KarolWelt {
        let mut welt = KarolWelt { breite: breite, laenge: laenge, hoehe: hoehe, feld: Vec::new() };

        for x in 0..welt.breite {
            welt.feld.push(Vec::new());
            for _y in 0..welt.laenge {
                welt.feld[x].push(WeltPosition::new(welt.hoehe));
            }
        }

        return welt;
    }

    pub fn _from_string(eingabe: &String) -> Option<KarolWelt> {
        let mut eingabe_iter = eingabe.split(' ');

        if eingabe_iter.next().unwrap() != VERSION_STRING {
            return None;
        }

        let breite = eingabe_iter.next().unwrap().parse::<usize>().unwrap();
        let laenge = eingabe_iter.next().unwrap().parse::<usize>().unwrap();
        let hoehe = eingabe_iter.next().unwrap().parse::<usize>().unwrap();

        eingabe_iter.next();
        eingabe_iter.next();
        eingabe_iter.next();

        let mut ergebnis = KarolWelt::new(breite, laenge, hoehe);

        for x in 0..ergebnis.breite {
            for y in 0..ergebnis.laenge {
                for h in 0..ergebnis.hoehe {
                    ergebnis.feld[x][y].ziegel[h] = Ziegel::from_char(eingabe_iter.next().unwrap().parse::<char>().unwrap()).unwrap();
                }
                ergebnis.feld[x][y].marke = Marke::from_char(eingabe_iter.next().unwrap().parse::<char>().unwrap()).unwrap();
            }
        }

        eingabe_iter.next();
        eingabe_iter.next();
        eingabe_iter.next();

        return None;
    }

    pub fn to_string(&self) -> String {
        let mut ergebnis = String::new();

        ergebnis.push_str(VERSION_STRING);
        ergebnis.push(' ');
        ergebnis.push_str(self.breite.to_string().as_str());
        ergebnis.push(' ');
        ergebnis.push_str(self.laenge.to_string().as_str());
        ergebnis.push(' ');
        ergebnis.push_str(self.hoehe.to_string().as_str());
        ergebnis.push(' ');
        ergebnis.push_str("0 0 0 ");

        for x in 0..self.breite {
            for y in 0..self.laenge {
                for ziegel in &self.feld[x][y].ziegel {
                    ergebnis.push(ziegel.clone() as u8 as char);
                    ergebnis.push(' ');
                }
                ergebnis.push(self.feld[x][y].marke.clone() as u8 as char);
                ergebnis.push(' ');
            }
        }

        ergebnis.push_str("1 1 S");

        return ergebnis;
    }
}