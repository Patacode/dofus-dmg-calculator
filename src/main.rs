use clap::{CommandFactory, Parser};

#[derive(Parser, Debug)]
#[command(version, about, author, long_about = None)]
#[command(help_template = "\
{before-help}
{usage-heading} {usage}

{about-with-newline}
{all-args}{after-help}

Version: {version}
Author: {author}
")]
struct Args {
    #[arg(
        short = 'i',
        long = "default-min",
        default_value_t = 0,
        help = "The minimum default damage of your spell"
    )]
    dmin: u64,

    #[arg(
        short = 'j',
        long = "default-max",
        default_value_t = 0,
        help = "The maximum default damage of your spell"
    )]
    dmax: u64,

    #[arg(
        short = 'k',
        long = "crit-min",
        default_value_t = 0,
        help = "The minimum critical damage of your spell"
    )]
    cmin: u64,

    #[arg(
        short = 'l',
        long = "crit-max",
        default_value_t = 0,
        help = "The maximum critical damage of your spell"
    )]
    cmax: u64,

    #[arg(
        short,
        long,
        default_value_t = 0,
        help = "The stat points of your spell's type your character has"
    )]
    stat: u64,

    #[arg(short, long, default_value_t = 0, help = "The power of your character")]
    power: u64,

    #[arg(
        short = 'f',
        long = "fixed-dmg",
        default_value_t = 0,
        help = "The fixed damage of your spell's type your character has"
    )]
    dfixed: u64,

    #[arg(short = 'a', long = "author", action = clap::ArgAction::SetTrue, help = "Print author infos")]
    show_author: bool,
}

fn calculate_damage(base_damage: u64, relevant_stat: u64, power: u64, fixed_bonus: u64) -> u64 {
    let multiplier = 1.0 + (relevant_stat + power) as f64 / 100.0;
    let total = base_damage as f64 * multiplier + fixed_bonus as f64;
    total.floor() as u64
}

fn main() {
    let args = Args::parse();

    if args.show_author {
        let cmd = Args::command();
        if let Some(author) = cmd.get_author() {
            println!("{author}");
        } else {
            println!("Author information not available.");
        }
        return;
    }

    let dmg_dmin = calculate_damage(args.dmin, args.stat, args.power, args.dfixed);
    let dmg_dmax = calculate_damage(args.dmax, args.stat, args.power, args.dfixed);
    let dmg_cmin = calculate_damage(args.cmin, args.stat, args.power, args.dfixed);
    let dmg_cmax = calculate_damage(args.cmax, args.stat, args.power, args.dfixed);

    println!("Damage estimation = {dmg_dmin} - {dmg_dmax} ({dmg_cmin} - {dmg_cmax})")
}
