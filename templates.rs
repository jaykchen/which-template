use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash)]
enum PromptType {
    Llama2Chat,
    Chatml,
    Openchat,
    Zephyr,
    CodellamaInstruct,
    MistralInstruct,
    StabllmZephyr,
    Mistrallite,
    Vicuna10Chat, // Vicuna-1.0-Chat
    Vicuna11Chat, // Vicuna-1.1-Chat
    WizardCoder,
    DeepseekChat,
    DeepseekCoder,
    SolarInstruct,
    IntelNeural,
    HumanAssistant,
}

// Define a struct to hold a template
#[derive(Debug)]
struct Template {
    prompt_type: PromptType,
    format: String,
}

// The main dictionary
let prompt_templates: HashMap<PromptType, Template> = HashMap::from([
    (
        PromptType::Llama2Chat,
        Template {
            prompt_type: PromptType::Llama2Chat,
            format: "<TEXT>\n<s>[INST] <<SYS>>{{ system_prompt }}<</SYS>>{{ user_msg_1 }} [/INST] {{ model_answer_1 }} </s><s>[INST] {{ user_msg_2 }} [/INST]".to_string(),
        },
    ),
    (
        PromptType::Chatml,
        Template {
            prompt_type: PromptType::Chatml,
            format: "<TEXT>\n<|im_start|>system{system_message}<|im_end|>\n<|im_start|>user{prompt}<|im_end|>\n<|im_start|>assistant".to_string(),
        },
    ),
    (
        PromptType::Openchat,
        Template {
            prompt_type: PromptType::Openchat,
            format: "<TEXT>\nGPT4 User: {prompt}<|end_of_turn|>GPT4 Assistant:".to_string(),
        },
    ),
    (
        PromptType::Zephyr,
        Template {
            prompt_type: PromptType::Zephyr,
            format: "<TEXT>\n<|system|>{system_prompt}</s>\n<|user|>{prompt}</s>\n<|assistant|>".to_string(),
        },
    ),
    (
        PromptType::CodellamaInstruct,
        Template {
            prompt_type: PromptType::CodellamaInstruct,
            format: "<TEXT>\n[INST] Write code to solve the following coding problem that obeys the constraints and passes the example test cases. Please wrap your code answer using ```:\n{prompt}\n[/INST]".to_string(),
        },
    ),
    (
        PromptType::MistralInstruct,
        Template {
            prompt_type: PromptType::MistralInstruct,
            format: "<TEXT>\n<s>[INST] {prompt} [/INST]".to_string(),
        },
    ),
    (
        PromptType::StabllmZephyr,
        Template {
            prompt_type: PromptType::StabllmZephyr,
            format: "<TEXT>\n<|user|>{prompt}<|endoftext|>\n<|assistant|>".to_string(),
        },
    ),
    (
        PromptType::Mistrallite,
        Template {
            prompt_type: PromptType::Mistrallite,
            format: "<TEXT>\n<|prompter|>{prompt}</s><|assistant|>".to_string(),
        },
    ),
    (
        PromptType::Vicuna10Chat,
        Template {
            prompt_type: PromptType::Vicuna10Chat,
            format: "<TEXT>\n{system} USER: {prompt} ASSISTANT:".to_string(),
        },
    ),
    (
        PromptType::Vicuna11Chat,
        Template {
            prompt_type: PromptType::Vicuna11Chat,
            format: "<TEXT>\nUSER: {prompt}\nASSISTANT:".to_string(),
        },
    ),
    (
        PromptType::WizardCoder,
        Template {
            prompt_type: PromptType::WizardCoder,
            format: "<TEXT>\n{system}\n### Instruction:\n{instruction}\n### Response:".to_string(),
        },
    ),
    (
        PromptType::DeepseekChat,
        Template {
            prompt_type: PromptType::DeepseekChat,
            format: "<TEXT>\nUser: {user_message_1}\nAssistant: {assistant_message_1}<｜end of sentence｜>User: {user_message_2}\nAssistant:".to_string(),
        },
    ),
    (
        PromptType::DeepseekCoder,
        Template {
            prompt_type: PromptType::DeepseekCoder,
            format: "<TEXT>\n{system}\n### Instruction:\n{question_1}\n### Response:\n{answer_1}\n<|EOT|>\n### Instruction:\n{question_2}\n### Response:".to_string(),
        },
    ),
    (
        PromptType::SolarInstruct,
        Template {
            prompt_type: PromptType::SolarInstruct,
            format: "<TEXT>\n### User:\n{prompt}\n### Assistant:".to_string(),
        },
    ),
    (
        PromptType::IntelNeural,
        Template {
            prompt_type: PromptType::IntelNeural,
            format: "<TEXT>\n### System:\n{system}\n### User:\n{usr}\n### Assistant:".to_string(),
        },
    ),
    (
        PromptType::HumanAssistant,
        Template {
            prompt_type: PromptType::HumanAssistant,
            format: "<TEXT>\nHuman: {input_1}\n\nAssistant:{output_1}Human: {input_2}\n\nAssistant:".to_string(),
        },
    ),
]);
