use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use minsk_language::code_analysis::{
    compilation::Compilation, evaluation_result::EvaluationResult, minsk_value::MinskValue,
    syntax::syntax_tree::SyntaxTree, variable_symbol::VariableSymbol,
};
use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader, Write},
};

fn main() -> anyhow::Result<()> {
    let mut stdout = io::stdout();
    let mut reader = BufReader::new(io::stdin());
    let mut line = String::new();
    let mut show_tree = true;
    let mut variables = HashMap::<VariableSymbol, MinskValue>::new();

    loop {
        line.clear();
        print!("minsk:> ");
        stdout.flush()?;
        if reader.read_line(&mut line)? == 0 {
            break;
        }

        match line.trim() {
            "#cls" => {
                stdout.execute(Clear(ClearType::All))?;
                continue;
            }
            "#showTree" => {
                show_tree = !show_tree;
                println!(
                    "{}",
                    if show_tree {
                        "Showing parse trees"
                    } else {
                        "Not showing parse trees"
                    }
                );
                continue;
            }
            _ => {}
        }

        let tree = SyntaxTree::parse(line.trim().to_string());
        if show_tree {
            println!("{}", tree.root());
        }
        let evaluation_result = Compilation::evaluate(tree, &mut variables);
        match evaluation_result {
            EvaluationResult::Error(diagnostics) => {
                for diagnostic in diagnostics {
                    println!();
                    stdout.execute(SetForegroundColor(Color::DarkRed))?;
                    println!("{}", diagnostic);
                    stdout.execute(ResetColor)?;
                    let prefix = &line[0..diagnostic.span.start];
                    let error = &line[diagnostic.span.start..diagnostic.span.end];
                    let suffix = &line[diagnostic.span.end..];

                    print!("    {}", prefix);
                    stdout.execute(SetForegroundColor(Color::DarkRed))?;
                    print!("{}", error);
                    stdout.execute(ResetColor)?;
                    println!("{}", suffix);
                }
            }
            EvaluationResult::Value(value) => {
                println!("{}", value);
            }
        }
    }
    Ok(())
}
