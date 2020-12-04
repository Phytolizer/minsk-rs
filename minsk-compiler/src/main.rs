use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use minsk_language::code_analysis::{
    binding::binder::Binder, compilation::Compilation, evaluation_result::EvaluationResult,
    evaluator::Evaluator, syntax::syntax_tree::SyntaxTree,
};
use std::io::{self, BufRead, BufReader, Write};

fn main() -> anyhow::Result<()> {
    let mut stdout = io::stdout();
    let mut reader = BufReader::new(io::stdin());
    let mut line = String::new();
    let mut show_tree = true;

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
        let evaluation_result = Compilation::evaluate(tree);
        match evaluation_result {
            EvaluationResult::Error(diagnostics) => {
                stdout.execute(SetForegroundColor(Color::DarkRed))?;
                for diagnostic in diagnostics {
                    println!("{}", diagnostic);
                }
                stdout.execute(ResetColor)?;
            }
            EvaluationResult::Value(value) => {
                println!("{}", value);
            }
        }
    }
    Ok(())
}
