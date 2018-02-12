pub mod error;

use alg::moves::Move;
use cube::turns::Turn;
use self::error::AlgParseError;
use std::error::Error;
use std::str::FromStr;

#[derive(Default)]
pub struct Parser {
    input: String,
    result: Vec<Result<Move, AlgParseError>>,
}

pub struct Generator {
    from: Option<Vec<Move>>,
}

impl Parser {
    fn flush(&mut self) {
        self.input.clear();
        self.result.clear();
    }

    pub fn new() -> Self {
        Self {
            input: String::with_capacity(30),
            result: Vec::with_capacity(30),
        }
    }

    pub fn parse(&mut self, input: &str) -> &Self {
        self.flush();

        let res = input
            .split_whitespace()
            .map(Move::from_str)
            .collect::<Vec<_>>();

        self.result.extend(res);
        self.input.push_str(input);
        self
    }

    pub fn report(&self) -> Generator {
        let mut has_err = false;
        let mut gen = Vec::new();

        for res in self.result.iter().zip(self.input.split_whitespace()) {
            match res {
                (&Ok(ref m), _) => gen.push(m.clone()),
                (&Err(ref e), inp) => {
                    has_err = true;
                    println!(
                        "For input `{}` => {}",
                        inp,
                        e.cause().unwrap().description()
                    );
                }
            }
        }
        if !has_err {
            Generator { from: Some(gen) }
        } else {
            Generator { from: None }
        }
    }
}

impl Generator {
    pub fn generate(self) -> Option<Vec<Turn>> {
        if self.from.is_none() {
            return None;
        }

        let moves = self.from.unwrap();
        let mut turns = Vec::new();

        for m in moves {
            let from_moves: Vec<Turn> = m.into();
            turns.extend(from_moves);
        }
        Some(turns)
    }
}
