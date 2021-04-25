use std::{ fs::File, io::{BufWriter, Write}, };
use crate::ProcessedLine;

pub fn write_dns_config(lines: &[ProcessedLine]) {
    let file = File::create("./db.ionescu").unwrap();
    let mut out = BufWriter::new(&file);

    let longest = compute_max_name_lenght(lines);
    for line in lines {
        if let ProcessedLine::Line { number, text, mac, ip, names } = line {
            write_ip_group(&mut out, ip, &names, longest);
        }
    }
}

fn write_ip_group(out: &mut BufWriter<&File>, ip: &str, names: &[&str], longest: usize ) {
    let mut i = names.into_iter();
    let a = i.next().unwrap();
    let width = longest+1;
    writeln!(out, "{a:width$} IN A     {ip}");
    i.for_each(|name| { writeln!(out, "{0:1$} IN CNAME {2}", name, longest + 1, a); });
}

fn compute_max_name_lenght(lines: &[ProcessedLine]) -> usize {
    lines.iter()
        .map(|x| match x {
            ProcessedLine::Line { number, text, mac, ip, names } => 
                names.iter().map(|s| s.len()).max().unwrap(),
            _ => 0
        }).max().unwrap()
}
