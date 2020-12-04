use code_analysis::{
    binding::binder::Binder, evaluator::Evaluator, syntax::syntax_node::SyntaxNode,
    syntax::syntax_tree::SyntaxTree,
};
use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::io::{self, BufRead, BufReader, Write};
mod code_analysis;
mod minsk_value;

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
        let mut diagnostics = tree.diagnostics().to_owned();
        let mut binder = Binder::new();
        let bound_expression = binder.bind(tree.root());
        let mut diagnostics2 = binder.diagnostics().to_vec();
        diagnostics.append(&mut diagnostics2);
        drop(diagnostics2);
        if show_tree {
            println!("{}", tree.root());
        }
        if tree.diagnostics().len() > 0 {
            stdout.execute(SetForegroundColor(Color::DarkRed))?;
            for diagnostic in tree.diagnostics() {
                println!("{}", diagnostic);
            }
            stdout.execute(ResetColor)?;
        } else if let SyntaxNode::ExpressionSyntax(e) = tree.root() {
            let result = Evaluator::evaluate(&bound_expression);
            println!("{}", result);
        }
    }
    Ok(())
}
