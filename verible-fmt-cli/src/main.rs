use clap::{Arg, Command};
use std::fs;
use verible_fmt_core::VerilogFormatter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let matches = Command::new("verible-fmt")
        .version("0.1.0")
        .about("A Verilog/SystemVerilog formatter")
        .arg(
            Arg::new("input")
                .help("Input file to format")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Output file (defaults to stdout)")
                .value_name("FILE"),
        )
        .get_matches();

    let input_file = matches.get_one::<String>("input").unwrap();
    let input_content = fs::read_to_string(input_file)?;

    let formatter = VerilogFormatter::new();
    let formatted = formatter.format(&input_content)?;

    if let Some(output_file) = matches.get_one::<String>("output") {
        fs::write(output_file, formatted)?;
    } else {
        print!("{}", formatted);
    }

    Ok(())
}