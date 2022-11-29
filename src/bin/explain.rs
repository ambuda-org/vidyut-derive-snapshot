use clap::Parser;
use std::error::Error;
use std::path::Path;
use vidyut_prakriya::ashtadhyayi as A;
use vidyut_prakriya::constants::{La, Prayoga, Purusha, Vacana};
use vidyut_prakriya::dhatupatha as D;
use vidyut_prakriya::prakriya::Prakriya;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long)]
    code: String,
    #[arg(long)]
    pada: String,
}

fn pretty_print_prakriya(p: &Prakriya) {
    println!("------------------------------");
    for step in p.history() {
        println!("{:<10} | {}", step.rule, step.state);
    }
    println!("------------------------------");
    for choice in p.rule_choices() {
        println!("{choice:?}");
    }
    println!("------------------------------");
}

const LAKARA: &[La] = &[
    La::Lat,
    La::Lit,
    La::Lut,
    La::Lrt,
    La::Lot,
    La::Lan,
    La::AshirLin,
    La::VidhiLin,
    La::Lun,
    La::Lrn,
];

const PURUSHA_VACANA: &[(Purusha, Vacana)] = &[
    (Purusha::Prathama, Vacana::Eka),
    (Purusha::Prathama, Vacana::Dvi),
    (Purusha::Prathama, Vacana::Bahu),
    (Purusha::Madhyama, Vacana::Eka),
    (Purusha::Madhyama, Vacana::Dvi),
    (Purusha::Madhyama, Vacana::Bahu),
    (Purusha::Uttama, Vacana::Eka),
    (Purusha::Uttama, Vacana::Dvi),
    (Purusha::Uttama, Vacana::Bahu),
];

fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let dhatus = D::load_dhatus(Path::new("data/dhatupatha.tsv"));

    let mut words = vec![];
    for dhatu in dhatus?.iter() {
        if dhatu.code() != args.code {
            continue;
        }
        for la in LAKARA {
            for (purusha, vacana) in PURUSHA_VACANA {
                let ps = A::derive_tinantas(
                    &dhatu.upadesha,
                    &dhatu.code(),
                    *la,
                    Prayoga::Kartari,
                    *purusha,
                    *vacana,
                );
                for p in ps {
                    words.push(p.text());
                    if p.text() == args.pada {
                        println!("{:?} {:?} {:?}", la, purusha, vacana);
                        pretty_print_prakriya(&p);
                    }
                }
            }
        }
    }

    println!("{}", words.join(", "));
    Ok(())
}

fn main() {
    let args = Args::parse();

    match run(args) {
        Ok(()) => (),
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    }
}
