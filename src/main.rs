use clap::CommandFactory;
use clap::Parser;

use dofus_dmg_calculator::compute_dmg_estimation_with_res;

#[derive(Parser, Debug)]
#[command(version, author, long_about = None)]
#[command(about = "Compute a Dofus spell damage estimation")]
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
        help = "The spell minimum default damage"
    )]
    min_default_dmg: u64,

    #[arg(
        short = 'j',
        long = "default-max",
        default_value_t = 0,
        help = "The spell maximum default damage"
    )]
    max_default_dmg: u64,

    #[arg(
        short = 'k',
        long = "crit-min",
        default_value_t = 0,
        help = "The spell minimum critical damage"
    )]
    min_crit_dmg: u64,

    #[arg(
        short = 'l',
        long = "crit-max",
        default_value_t = 0,
        help = "The spell maximum critical damage"
    )]
    max_crit_dmg: u64,

    #[arg(
        short,
        long,
        default_value_t = 0,
        help = "The character stat points"
    )]
    stat_points: u64,

    #[arg(short, long, default_value_t = 0, help = "The character power")]
    power: u64,

    #[arg(
        short,
        long,
        default_value_t = 0,
        help = "The character fixed damage"
    )]
    fixed_dmg: u64,

    #[arg(
        short = 'x',
        long,
        default_value_t = 0,
        help = "The enemy fixed resistance"
    )]
    fixed_res: u64,

    #[arg(
        short = 'r',
        long,
        default_value_t = 0,
        help = "The enemy variable resistance"
    )]
    variable_res: u64,

    #[arg(
        short = 'a',
        long = "author",
        action = clap::ArgAction::SetTrue,
        help = "Print author"
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

    let min_default_dmg = compute_dmg_estimation_with_res(
        args.min_default_dmg,
        args.fixed_dmg,
        args.stat_points,
        args.power,
        args.fixed_res,
        args.variable_res,
    );
    let max_default_dmg = compute_dmg_estimation_with_res(
        args.max_default_dmg,
        args.fixed_dmg,
        args.stat_points,
        args.power,
        args.fixed_res,
        args.variable_res,
    );
    let min_crit_dmg = compute_dmg_estimation_with_res(
        args.min_crit_dmg,
        args.fixed_dmg,
        args.stat_points,
        args.power,
        args.fixed_res,
        args.variable_res,
    );
    let max_crit_dmg = compute_dmg_estimation_with_res(
        args.max_crit_dmg,
        args.fixed_dmg,
        args.stat_points,
        args.power,
        args.fixed_res,
        args.variable_res,
    );

    println!(
        "Damage estimation = {min_default_dmg} - {max_default_dmg} ({min_crit_dmg} - {max_crit_dmg})"
    )
}
