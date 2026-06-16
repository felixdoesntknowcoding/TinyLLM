use std::io::{self, Write};

/// A tiny, theoretical LLM-style response engine.
///
/// This is NOT a real trained neural network.
/// It demonstrates the safest part of an LLM pipeline:
/// prompt -> normalize -> retrieve known facts/rules -> answer only when confidence is enough.
///
/// The main anti-hallucination rule is simple:
/// if the engine cannot match the prompt to known information, it says it does not know.
#[derive(Debug, Clone)]
struct TinyLlm {
    memory: Vec<KnowledgeItem>,
    min_confidence: f32,
}

#[derive(Debug, Clone)]
struct KnowledgeItem {
    triggers: Vec<&'static str>,
    output: &'static str,
}

#[derive(Debug, Clone)]
struct Answer {
    text: String,
    confidence: f32,
    source: &'static str,
}

impl TinyLlm {
    fn new() -> Self {
        Self {
            min_confidence: 0.45,
            memory: vec![
                KnowledgeItem {
                    triggers: vec!["hello", "hi", "hey"],
                    output: "Hello. I am TinyLLM. Ask me a simple known question.",
                },
                KnowledgeItem {
                    triggers: vec!["what are you", "who are you", "tinyllm"],
                    output: "I am a tiny theoretical prompt-to-output engine written in Rust.",
                },
                KnowledgeItem {
                    triggers: vec!["rust", "fast language", "programming language"],
                    output: "Rust is a fast compiled programming language with memory safety and no garbage collector.",
                },
                KnowledgeItem {
                    triggers: vec!["hallucination", "not hallucinate", "avoid hallucination"],
                    output: "I avoid hallucination by only answering from my small built-in memory. Unknown prompts get an 'I don't know' answer.",
                },
                KnowledgeItem {
                    triggers: vec!["minimal", "simple output", "short answer"],
                    output: "Minimal mode: answer only what is known, with no extra guessing.",
                },
                KnowledgeItem {
                    triggers: vec!["help", "commands"],
                    output: "Try: hello, what are you, why Rust, avoid hallucination, minimal output, or exit.",
                },
            ],
        }
    }

    fn generate(&self, prompt: &str) -> Answer {
        let normalized = normalize(prompt);

        if normalized.trim().is_empty() {
            return Answer {
                text: "Please type a prompt.".to_string(),
                confidence: 1.0,
                source: "input-check",
            };
        }

        let mut best_output = None;
        let mut best_score = 0.0;

        for item in &self.memory {
            let score = item
                .triggers
                .iter()
                .map(|trigger| similarity(&normalized, trigger))
                .fold(0.0, f32::max);

            if score > best_score {
                best_score = score;
                best_output = Some(item.output);
            }
        }

        if best_score >= self.min_confidence {
            Answer {
                text: best_output.unwrap_or("I don't know.").to_string(),
                confidence: best_score,
                source: "built-in-memory",
            }
        } else {
            Answer {
                text: "I don't know. My memory has no reliable answer for that prompt.".to_string(),
                confidence: best_score,
                source: "abstain-rule",
            }
        }
    }
}

fn normalize(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|ch| if ch.is_alphanumeric() || ch.is_whitespace() { ch } else { ' ' })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Very small token-overlap similarity.
/// This is deliberately simple and explainable.
fn similarity(prompt: &str, trigger: &str) -> f32 {
    let prompt_tokens: Vec<&str> = prompt.split_whitespace().collect();
    let trigger_tokens: Vec<&str> = trigger.split_whitespace().collect();

    if prompt_tokens.is_empty() || trigger_tokens.is_empty() {
        return 0.0;
    }

    let matches = trigger_tokens
        .iter()
        .filter(|token| prompt_tokens.contains(token))
        .count();

    matches as f32 / trigger_tokens.len() as f32
}

fn main() {
    let model = TinyLlm::new();

    println!("TinyLLM theoretical Rust demo");
    println!("Type a prompt. Type 'exit' to stop.\n");

    loop {
        print!("> ");
        io::stdout().flush().expect("failed to flush stdout");

        let mut prompt = String::new();
        if io::stdin().read_line(&mut prompt).is_err() {
            println!("I don't know. Input could not be read.");
            continue;
        }

        let prompt = prompt.trim();
        if prompt.eq_ignore_ascii_case("exit") || prompt.eq_ignore_ascii_case("quit") {
            println!("Goodbye.");
            break;
        }

        let answer = model.generate(prompt);
        println!("{}", answer.text);
        println!("[confidence: {:.2}, source: {}]\n", answer.confidence, answer.source);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_prompt_gets_answer() {
        let model = TinyLlm::new();
        let answer = model.generate("How do you avoid hallucination?");
        assert!(answer.confidence >= model.min_confidence);
        assert!(answer.text.contains("avoid hallucination"));
    }

    #[test]
    fn unknown_prompt_abstains() {
        let model = TinyLlm::new();
        let answer = model.generate("Tell me tomorrow's lottery numbers");
        assert_eq!(answer.source, "abstain-rule");
        assert!(answer.text.contains("I don't know"));
    }
}
