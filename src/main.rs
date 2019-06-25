use clap::*;
use colored::*;
// use elfkit::symbol::SymbolSectionIndex;
use elfkit::Elf;
// use std::env;
use std::fs::File;

fn hextab<S>(align: usize, s: S) -> String
where
    S: std::fmt::LowerHex,
{
    let s = format!("{:.align$x}", s, align = align);
    let pad: String = vec!['0'; align - s.len()].into_iter().collect();
    format!("\x1b[90m{}\x1b[0;m{}", pad, s)
}

fn main() {
    let matches = clap_app!(diffelf =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: "Diff me some elfs")
        (@arg ELF1: +required "Difference elf")
        (@arg ELF2: +required "Base elf")
    )
    .get_matches();

    for input_file in ["ELF1", "ELF2"].iter() {
        let mut file = File::open(matches.value_of(input_file).unwrap()).unwrap();
        let mut elf = Elf::from_reader(&mut file).unwrap();
        elf.load_all(&mut file).unwrap();

        // Shamelessly pilfered from https://github.com/aep/elfkit
        println!(
            "{} {}",
            "\nInfo for".bold().blue(),
            input_file.bold().magenta()
        );
        println!(
            "{} at offset 0x{:x}:",
            "Section Headers".bold(),
            elf.header.shoff
        );
        println!(
            "  [Nr] Name             Type           Address          Offset   Size     EntS \
             Flg Lnk Inf Al"
        );

        for (i, section) in elf.sections.iter().enumerate() {
            println!(
                "  [{:>2}] {:<16.16} {} {} {} {} {} {:<3} {:<3.3} {:<3} {:<2.2}",
                i,
                String::from_utf8_lossy(&section.name).bold(),
                match section.header.shtype.typename(&elf.header) {
                    Some(s) => format!("{:<14.14}", s),
                    None => hextab(14, section.header.shtype.to_u32()),
                },
                hextab(16, section.header.addr),
                hextab(8, section.header.offset),
                hextab(8, section.header.size),
                hextab(4, section.header.entsize),
                section.header.flags,
                section.header.link,
                section.header.info,
                section.header.addralign
            );
        }
    }
}
