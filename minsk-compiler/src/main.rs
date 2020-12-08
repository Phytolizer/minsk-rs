use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use minsk_language::code_analysis::{
    compilation::Compilation, minsk_value::MinskValue, syntax::syntax_tree::SyntaxTree,
    text::text_span::TextSpan, variable_symbol::VariableSymbol,
};
use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader, Write},
};

fn main() -> anyhow::Result<()> {
    let mut stdout = io::stdout();
    let mut reader = BufReader::new(io::stdin());
    let mut line = String::new();
    let mut text_builder = String::new();
    let mut show_tree = false;
    let mut variables = HashMap::<VariableSymbol, MinskValue>::new();
    let mut previous: Option<Compilation> = None;

    loop {
        line.clear();
        print!("minsk");
        stdout.execute(SetForegroundColor(Color::Green))?;
        if text_builder.is_empty() {
            print!(":> ");
        } else {
            print!(":| ");
        }
        stdout.execute(ResetColor)?;
        stdout.flush()?;
        if reader.read_line(&mut line)? == 0 {
            break;
        }

        if text_builder.is_empty() {
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
                "#reset" => previous = None,
                _ => {}
            }
        }

        text_builder.push_str(&line);
        let text = text_builder.clone();
        let tree = SyntaxTree::parse(text.clone());

        if !line.trim().is_empty() && tree.diagnostics().count() > 0 {
            continue;
        }
        if show_tree {
            println!("{}", tree.root());
        }
        let mut compilation = if let Some(previous) = previous.clone() {
            previous.continue_with(tree.clone())
        } else {
            Compilation::new(tree.clone())
        };
        let evaluation_result = compilation.evaluate(&mut variables);
        match evaluation_result {
            Err(diagnostics) => {
                let text = tree.text();
                for diagnostic in diagnostics {
                    let line_index = text.get_line_index(diagnostic.span.start).unwrap();
                    let line_number = line_index + 1;
                    let line = tree.text().lines()[line_index];
                    let character = diagnostic.span.start - text.lines()[line_index].start() + 1;
                    println!();
                    stdout.execute(SetForegroundColor(Color::DarkRed))?;
                    print!("({}, {}): ", line_number, character);
                    println!("{}", diagnostic);
                    stdout.execute(ResetColor)?;
                    let prefix = &tree.text()[TextSpan {
                        start: line.start(),
                        end: diagnostic.span.start,
                    }]
                    .iter()
                    .collect::<String>();
                    let error = &tree.text()[diagnostic.span].iter().collect::<String>();
                    let suffix = &tree.text()[TextSpan {
                        start: diagnostic.span.end,
                        end: line.end(),
                    }]
                    .iter()
                    .collect::<String>();

                    print!("    {}", prefix);
                    stdout.execute(SetForegroundColor(Color::DarkRed))?;
                    print!("{}", error);
                    stdout.execute(ResetColor)?;
                    println!("{}", suffix);
                }
            }
            Ok(value) => {
                if let Some(v) = value {
                    stdout.execute(SetForegroundColor(Color::Magenta))?;
                    println!("{}", v);
                    stdout.execute(ResetColor)?;
                }
                previous = Some(compilation);
            }
        }

        text_builder.clear();
    }
    Ok(())
}
