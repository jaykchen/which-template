use std::collections::HashMap;
use regex::Regex;

// Define enum for signature elements
#[derive(Debug, Eq, PartialEq, Hash)]
enum SignatureElement {
    SystemDelimiter,
    UserDelimiter,
    ImStart,
    ImEnd,
    Prompter,
    Assistant,
    EndSYS,
    InstructionStart,
    InstructionEnd,
    EndOfTurn,
    EndOfText,
    EndOfSentence,
    Newline,
}

// Define a struct to hold a template
#[derive(Debug)]
struct Template {
    prompt_type: PromptType,
    format: String,
}

// Assuming you have your PromptType enum defined

type PatternList = Vec<SignatureElement>;

// Populate prompt_templates dictionary (add your templates)
let prompt_templates: HashMap<PromptType, Template> = HashMap::from([
    // ...
]);

// Create a map to associate SignatureElement with patterns
let pattern_map: HashMap<SignatureElement, &str> = HashMap::from([
    (SignatureElement::SystemDelimiter, "<|system|>"),
    (SignatureElement::UserDelimiter, "<|user|>"),
    (SignatureElement::ImStart, "<|im_start|>"),
    (SignatureElement::ImEnd, "<|im_end|>"),
    (SignatureElement::Prompter, "<|prompter|>"),
    (SignatureElement::Assistant, "<|assistant|>"),
    (SignatureElement::EndSYS, "</SYS>"),
    (SignatureElement::InstructionStart, "[INST]"),
    (SignatureElement::InstructionEnd, "[/INST]"),
    (SignatureElement::EndOfTurn, "<|end_of_turn|>"),
    (SignatureElement::EndOfText, "<|EOT|>"),
    (SignatureElement::EndOfSentence, "<｜end of sentence｜>"),
    (SignatureElement::Newline, "\n"),
]);

// Build the pattern dictionary
let mut pattern_dictionary: HashMap<PromptType, PatternList> = HashMap::new();

for (prompt_type, template) in &prompt_templates {
    let mut patterns: PatternList = Vec::new();

    for cap in pattern_map.keys() {
        if template.format.contains(pattern_map[cap]) {
            patterns.push(*cap);
        }
    }
    pattern_dictionary.insert(*prompt_type, patterns);
}

// Pattern matching function (slightly modified)
fn has_pattern(template: &str, pattern: &SignatureElement) -> bool {
    if let Some(pattern_str) = pattern_map.get(pattern) {
        template.contains(pattern_str)
    } else {
        false
    }
}

// Example usage
let new_template = "<TEXT>\n<|user|>Hello there</|user|>\n<|assistant|>";
for (prompt_type, patterns) in &pattern_dictionary {
    if patterns.iter().any(|p| has_pattern(new_template, p)) {
        println!("The new template might be similar to: {:?}", prompt_type);
    }
}
