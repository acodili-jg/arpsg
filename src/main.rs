use std::{
    cmp::Ordering,
    io::{stdin, BufRead},
};

fn main() -> std::io::Result<()> {
    println!("Another Rock Paper Scissors Game 0.1.0");
    println!("Choose your weapon: rock, paper, or scissors.");

    let user_wpn = read_weapon()?;
    let opnt_wpn = rand::random();

    match user_wpn.cmp(&opnt_wpn) {
        Ordering::Less => println!("Your {user_wpn} lost to your opponent's {opnt_wpn}!"),
        Ordering::Equal => println!("It's a draw!"),
        Ordering::Greater => println!("Your {user_wpn} won to your opponent's {opnt_wpn}!"),
    };

    Ok(())
}

fn read_weapon() -> std::io::Result<Weapon> {
    let mut buffer = String::new();
    loop {
        stdin().lock().read_line(&mut buffer)?;
        match buffer.trim().parse() {
            Ok(user_wpn) => break Ok(user_wpn),
            Err(_) => buffer.clear(),
        };
    }
}

#[derive(
    Clone, Copy, Debug, Eq, PartialEq, rand_derive2::RandGen, strum::Display, strum::EnumString,
)]
#[strum(ascii_case_insensitive, serialize_all = "lowercase")]
enum Weapon {
    Rock,
    Paper,
    Scissors,
}

macro_rules! weapon_cmp_impl {
    ($lhs:ident, $mhs:ident, $rhs:ident) => {
        weapon_cmp_impl! {
            $lhs < $mhs,
            $mhs < $rhs,
            $rhs < $lhs,
        }
    };
    ($($lhs:ident < $rhs:ident,)+) => {
        weapon_cmp_impl! {$(
            $lhs <=> $rhs is Less,
            $lhs <=> $lhs is Equal,
            $rhs <=> $lhs is Greater,
        )+}
    };
    ($($lhs:ident <=> $rhs:ident is $ord:ident,)+) => {
        impl Ord for Weapon {
            #[inline]
            fn cmp(&self, rhs: &Weapon) -> Ordering {
                match (*self, *rhs) {$(
                    (Self::$lhs, Self::$rhs) => Ordering::$ord,
                )+}
            }
        }
    };
}

weapon_cmp_impl! { Rock, Paper, Scissors }

impl PartialOrd for Weapon {
    #[inline]
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}
