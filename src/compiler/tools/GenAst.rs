use std::fs::File;
use std::io::{BufWriter, Write};

fn define_ast(output_dir: &str, base_name: &str, types: &[&str]) {
    let path = format!("{}/{}.rs", output_dir, base_name);
    let mut file = std::fs::File::create(&path).expect("Unable to create file");
    let mut writer = BufWriter::new(file); // Import necessary types
    writeln!(writer, "use crate::compiler::token::Token;").unwrap();
    writeln!(writer, "use crate::compiler::expr::Object;\n").unwrap();

    // Define the main enum
    writeln!(writer, "#[derive(Debug, Clone)]").unwrap();
    writeln!(writer, "pub enum {} {{", base_name).unwrap();
    for type_ref in types {
        let struct_name = type_ref.split(':').next().unwrap().trim();
        writeln!(writer, "    {}(Box<{}>),", struct_name, struct_name).unwrap();
    }
    writeln!(writer, "}}\n").unwrap();

    // Define visitor trait
    writeln!(writer, "pub trait {}Visitor<T> {{", base_name).unwrap();
    for type_ref in types {
        let struct_name = type_ref.split(':').next().unwrap().trim();
        writeln!(
            writer,
            "    fn visit_{}(&self, {}: &{}) -> T;",
            struct_name.to_lowercase(),
            struct_name.to_lowercase(),
            struct_name
        )
        .unwrap();
    }
    writeln!(writer, "}}\n").unwrap();

    // Implement accept method for main enum
    writeln!(writer, "impl {} {{", base_name).unwrap();
    writeln!(
        writer,
        "    pub fn accept<T>(&self, visitor: &dyn {}Visitor<T>) -> T {{",
        base_name
    )
    .unwrap();
    writeln!(writer, "        match self {{").unwrap();
    for type_ref in types {
        let struct_name = type_ref.split(':').next().unwrap().trim();
        writeln!(
            writer,
            "            {}::{}(b) => visitor.visit_{}(b),",
            base_name,
            struct_name,
            struct_name.to_lowercase()
        )
        .unwrap();
    }
    writeln!(writer, "        }}").unwrap();
    writeln!(writer, "    }}").unwrap();
    writeln!(writer, "}}\n").unwrap();

    // Define individual structs
    for type_ref in types {
        define_type(&mut writer, type_ref);
    }
}

fn define_type(writer: &mut BufWriter<File>, type_def: &str) {
    let parts: Vec<&str> = type_def.split(':').collect();
    let struct_name = parts[0].trim();
    let fields = parts[1].trim();

    writeln!(writer, "#[derive(Debug, Clone)]").unwrap();
    writeln!(writer, "pub struct {} {{", struct_name).unwrap();

    for field in fields.split(',') {
        let field_parts: Vec<&str> = field.trim().split_whitespace().collect();
        let field_type = field_parts[0];
        let field_name = field_parts[1];

        // Handle Box<Expr> case
        let field_type = match field_type {
            "Expr" => "Box<Expr>",
            _ => field_type,
        };

        writeln!(writer, "    pub {}: {},", field_name, field_type).unwrap();
    }

    writeln!(writer, "}}\n").unwrap();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: generate_ast <output_directory>");
        std::process::exit(1);
    }

    let output_dir = &args[1];

    define_ast(
        output_dir,
        "expr",
        &[
            "Binary   : Expr left, Token operator, Expr right",
            "Grouping : Expr expression",
            "Literal  : Object value",
            "Unary    : Token operator, Expr right",
            "Ternary  : Expr condition, Expr true_branch, Expr false_branch",
        ],
    );

    define_ast(
        output_dir,
        "stmt",
        &[
            "Expression : Expr expression",
            "Print      : Expr expression",
        ],
    );
}
