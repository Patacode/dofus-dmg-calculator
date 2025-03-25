use clap::CommandFactory;
use clap::Parser;
use dofus_dmg_calculator::compute_dmg_estimation;

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

    #[arg(
        short,
        long,
        default_value_t = 0,
        help = "The power of your character"
    )]
    power: u64,

    #[arg(
        short = 'f',
        long = "fixed-dmg",
        default_value_t = 0,
        help = "The fixed damage of your spell's type your character has"
    )]
    dfixed: u64,

    #[arg(
        short = 'a',
        long = "author",
        action = clap::ArgAction::SetTrue, help = "Print author infos"
    )]
    show_author: bool,
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

    let dmg_dmin =
        compute_dmg_estimation(args.dmin, args.dfixed, args.stat, args.power);
    let dmg_dmax =
        compute_dmg_estimation(args.dmax, args.dfixed, args.stat, args.power);
    let dmg_cmin =
        compute_dmg_estimation(args.cmin, args.dfixed, args.stat, args.power);
    let dmg_cmax =
        compute_dmg_estimation(args.cmax, args.dfixed, args.stat, args.power);

    println!(
        "Damage estimation = {dmg_dmin} - {dmg_dmax} ({dmg_cmin} - {dmg_cmax})"
    )
}
