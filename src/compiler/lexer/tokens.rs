use std::{path::PathBuf, process::exit};

use crate::compiler::parser::{Expression, Node};

use super::TokenInfo;
use std::{iter::Peekable, vec::IntoIter};

#[derive(Debug)]
pub struct Tokens {
    lines: Vec<String>,
    file_path: PathBuf,

    current: Option<TokenInfo>,
    starts: Vec<TokenInfo>,
    tokens: Peekable<IntoIter<TokenInfo>>,
}
impl Tokens {
    pub fn new(file_path: PathBuf, tokens: Vec<TokenInfo>, lines: Vec<String>) -> Self {
        return Self {
            file_path,
            starts: Vec::new(),
            current: None,
            lines,
            tokens: tokens.into_iter().peekable(),
        };
    }
    pub fn throw_error<T: ToString, E: ToString>(&mut self, message: T, notice: E) -> ! {
        let current = self.current.clone().unwrap();

        let line = self.lines.get(current.lines.start - 1).unwrap();
        println!("error: {}", message.to_string());
        println!(
            "  --> {}:{}:{}",
            self.file_path.to_string_lossy(),
            current.lines.start,
            current.columns.start
        );

        println!("  |");
        println!("  | {}", line);
        println!(
            "  | {}{} {}",
            " ".repeat(current.columns.start - 1),
            "^".repeat(current.columns.end - current.columns.start),
            notice.to_string()
        );
        exit(1)
    }
    pub fn create_node(&mut self, node: Node) -> Node {
        let start = self.starts.pop().unwrap_or_else(|| {
            panic!("No starting node for: {:#?}", node);
        });
        node
    }
    pub fn create_expression(&mut self, expression: Expression) -> Expression {
        let start = self.starts.pop().unwrap_or_else(|| {
            panic!("No starting node for: {:#?}", expression);
        });
        expression
    }
    pub fn start(&mut self) -> TokenInfo {
        let token = self.advance();
        self.starts.push(token.clone());
        token
    }
    pub fn advance(&mut self) -> TokenInfo {
        match self.tokens.next() {
            Some(info) => {
                self.current = Some(info.clone());
                info
            }
            None => todo!(),
        }
    }
    pub fn peek(&mut self) -> &TokenInfo {
        return match self.tokens.peek() {
            Some(info) => info,
            None => todo!(),
        };
    }
}