# TinyLLM

A tiny theoretical LLM-style prompt-to-output engine written in Rust.

This project is intentionally minimal. It is not a real trained neural network and does not pretend to know things it was not given. The goal is to demonstrate a safe tiny model pipeline:

```text
prompt -> normalize -> match known memory -> answer or abstain
```

## Why Rust?

Rust is a fast compiled language with strong memory safety and no garbage collector. That makes it a good language for experiments where speed, low memory use, and reliability matter.

## Anti-hallucination design

TinyLLM avoids hallucination by using a strict rule:

> If the prompt does not match known memory with enough confidence, answer: `I don't know.`

This makes the output minimal and predictable instead of creative or made up.

## Run

Install Rust, then run:

```bash
cargo run
```

Example prompts:

```text
hello
what are you
why Rust?
how do you avoid hallucination?
minimal output
Tell me tomorrow's lottery numbers
```

## Test

```bash
cargo test
```

## Example output

```text
> how do you avoid hallucination?
I avoid hallucination by only answering from my small built-in memory. Unknown prompts get an 'I don't know' answer.
[confidence: 1.00, source: built-in-memory]

> Tell me tomorrow's lottery numbers
I don't know. My memory has no reliable answer for that prompt.
[confidence: 0.00, source: abstain-rule]
```

## Next ideas

- Load memory from a file instead of hard-coding it.
- Add a tiny tokenizer.
- Add embeddings or keyword weights.
- Add a small transformer later, while keeping the abstain rule.
- Add a confidence threshold setting.
