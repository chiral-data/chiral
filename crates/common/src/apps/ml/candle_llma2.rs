// candle llma2 
// https://github.com/huggingface/candle/tree/main/candle-wasm-examples/llama2-c

use serde::{Serialize, Deserialize};
use chiral_derive::*;
use crate::traits::*;

/// Input
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Serialization, InputFileRequirements)] 
pub struct Input {
    pub temperature: f64,
    pub prompt: String 
}

impl TraitInput for Input {
    fn default() -> Self {
        Self { temperature: 0.0, prompt: "Emma has a close friend".to_string() } 
    }
}

/// Output
#[derive(Debug, PartialEq, Serialize, Deserialize, Serialization, Clone)]
pub struct Output {
    pub text: String 
}

impl TraitOutput for Output {
    fn blank() -> Self { Self { text: "".to_string() } }

    fn len(&self) -> usize { panic!("not applicable") }

    fn clear(&mut self) { self.text.clear(); }

    fn append(&mut self, other: &mut Self) {
        self.text = other.text.to_string();
    }
}