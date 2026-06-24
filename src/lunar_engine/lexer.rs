// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

/// Lunar词法分析器 - 接口占位
#[derive(Debug, Clone)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Number(String),
    String(String),
    Symbol(char),
    Comment(String),
    Whitespace,
    Unknown,
}

pub struct Lexer {
    source: String,
    position: usize,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            source: String::new(),
            position: 0,
        }
    }

    pub fn tokenize(&mut self, source: &str) -> Vec<Token> {
        self.source = source.to_string();
        self.position = 0;
        let mut tokens = Vec::new();

        while self.position < self.source.len() {
            let ch = self.source.chars().nth(self.position).unwrap_or(' ');

            if ch.is_whitespace() {
                self.position += 1;
                continue;
            }

            if ch.is_alphabetic() || ch == '_' {
                let start = self.position;
                while self.position < self.source.len() {
                    let c = self.source.chars().nth(self.position).unwrap_or(' ');
                    if c.is_alphanumeric() || c == '_' {
                        self.position += 1;
                    } else {
                        break;
                    }
                }
                let word = &self.source[start..self.position];
                tokens.push(Token::Identifier(word.to_string()));
                continue;
            }

            if ch.is_ascii_digit() {
                let start = self.position;
                while self.position < self.source.len() {
                    let c = self.source.chars().nth(self.position).unwrap_or(' ');
                    if c.is_ascii_digit() || c == '.' {
                        self.position += 1;
                    } else {
                        break;
                    }
                }
                let num = &self.source[start..self.position];
                tokens.push(Token::Number(num.to_string()));
                continue;
            }

            if ch == '"' || ch == '\'' {
                let quote = ch;
                self.position += 1;
                let start = self.position;
                while self.position < self.source.len() {
                    let c = self.source.chars().nth(self.position).unwrap_or(' ');
                    if c == quote {
                        break;
                    }
                    self.position += 1;
                }
                let string = &self.source[start..self.position];
                tokens.push(Token::String(string.to_string()));
                self.position += 1;
                continue;
            }

            // 处理注释
            if ch == '/' && self.position + 1 < self.source.len() {
                let next = self.source.chars().nth(self.position + 1).unwrap_or(' ');
                if next == '/' {
                    let start = self.position;
                    while self.position < self.source.len() {
                        let c = self.source.chars().nth(self.position).unwrap_or(' ');
                        if c == '\n' || c == '\r' {
                            break;
                        }
                        self.position += 1;
                    }
                    let comment = &self.source[start..self.position];
                    tokens.push(Token::Comment(comment.to_string()));
                    continue;
                }
            }

            // 单个符号
            tokens.push(Token::Symbol(ch));
            self.position += 1;
        }

        tokens
    }
}
