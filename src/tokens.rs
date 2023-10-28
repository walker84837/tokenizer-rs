use tiktoken_rs::{p50k_base, cl100k_base, p50k_edit, r50k_base};

pub struct Tokenizer {

}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer { }
    }

    pub fn get_tokens(&mut self, mode: String, text: &str) -> Vec<usize> {
        let tokens: Vec<usize>;

        match mode.as_str() {
            "p50k_base" => { 
                let bpe = p50k_base().unwrap();
                tokens = bpe.encode_with_special_tokens(text);
            },

            "cl100k_base" => { 
                let bpe = cl100k_base().unwrap();
                tokens = bpe.encode_with_special_tokens(text);
            },

            "p50k_edit" => { 
                let bpe = p50k_edit().unwrap();
                tokens = bpe.encode_with_special_tokens(text);
            },

            "r50k_base" => { 
                let bpe = r50k_base().unwrap();
                tokens = bpe.encode_with_special_tokens(text);
            },

            _ => {
                panic!("Invalid mode. Choose between p50k_base, cl100k_base, p50k_edit, r50k_base.");
            }

        }

        tokens
    }
}

