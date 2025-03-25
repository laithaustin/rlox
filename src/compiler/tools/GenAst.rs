// goal of this is to generate our rust AST code without manually writing it
use std::fs::File;
use std::io::{self, BufWriter, Write};

// at a high level code should be generated like this:
// trait expr {}
// impl expr for binary {
//
fn define_ast(output_dir: &str, base_name: &str, types: &[&str]) {
    let path = format!("{}/{}.rs", output_dir, base_name);
    let mut file = std::fs::File::create(&path).expect("Unable to create file");
    let mut writer = BufWriter::new(file);

    // in java we would define a abstract class for expr but in rust we can use traits
    // we also need the equivalent of printwriter in rust, which is std::fs::File
    writeln!(writer, "trait {} {{", base_name).expect("Unable to write to file");
    writeln!(writer, "}}\n").expect("Unable to write to file");

    // now we need to define each of the structs/types
    for type_ref in types {
        define_type(&mut writer, base_name, type_ref);
    }
}

fn define_type(writer: &mut BufWriter<File>, base_name: &str, type_ref: &str) {
    // key idea here is to split on the fields and then generate the appropriate struct

    // split on the ':'
    let parts: Vec<&str> = type_ref.split(':').collect();
    let struct_name = parts[0].trim(); // get struct name without whitespace
    let body = parts[1].trim(); // get the fields without whitespace

    // first we generate the struct definition
    writeln!(writer, "struct {} {{", struct_name).expect("Unable to write to file");

    // now we need to split on the comma to get the individual fields
    let fields: Vec<&str> = body.split(',').collect();
    for field in fields.iter() {
        let field = field.trim();

        let field_parts: Vec<&str> = field.split_whitespace().collect();
        if field_parts.len() != 2 {
            panic!("Invalid field definition: {}", field);
        }
        writeln!(writer, "    pub {}: {},", field_parts[1], field_parts[0])
            .expect("Unable to write to file");
    }

    writeln!(writer, "}}\n").expect("Unable to write to file");
}

fn main() {
    //get args
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: generate_ast <output_directory>");
        std::process::exit(1);
    }
    let output_dir = &args[1];

    define_ast(
        output_dir,
        "Expr",
        &[
            "Binary   : Expr left, Token operator, Expr right",
            "Grouping : Expr expression",
            "Literal  : Object value",
            "Unary    : Token operator, Expr right",
        ],
    )
}
