use std::{
    cmp::Ordering,
    io::{stdin, BufRead},
};

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

fn main() -> std::io::Result<()> {
    print_header();
    let winning_result = play_while_tied()?;
    display_placings(winning_result);
    println!();
    Ok(())
}

#[inline]
fn print_header() {
    match VERSION {
        Some(version) => println!("Another Rock Paper Scissors Game {version}"),
        None => println!("Another Rock Paper Scissors Game"),
    };
}

#[inline]
fn play_while_tied() -> std::io::Result<Result<Weapon, Weapon>> {
    Ok(loop {
        println!("Choose your weapon: rock, paper, or scissors.");
        let opnt_wpn = rand::random();
        let user_wpn = read_weapon()?;
        match user_wpn.cmp(&opnt_wpn) {
            Ordering::Greater => break Ok(opnt_wpn),
            Ordering::Less => break Err(opnt_wpn),
            Ordering::Equal => {
                println!("It's a draw!");
                println!();
            }
        };
    })
}

#[inline]
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

#[inline]
fn display_placings(winning_result: Result<Weapon, Weapon>) {
    match winning_result {
        Ok(opnt_wpn) => println!("You won against your opponent's {opnt_wpn}!"),
        Err(opnt_wpn) => println!("You lost to your opponent's {opnt_wpn}!"),
    };
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
