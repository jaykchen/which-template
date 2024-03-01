
Prompt type	Format	Models using this template
llama-2-chat	
<s>[INST] <<SYS>>
{{ system_prompt }}
<</SYS>>

{{ user_msg_1 }} [/INST] {{ model_answer_1 }} </s><s>[INST] {{ user_msg_2 }}   [/INST]
Llama-2-7B-Chat, Llama-2-13B-Chat
chatml	
<|im_start|>system
{system_message}<|im_end|>
<|im_start|>user
{prompt}<|im_end|>
<|im_start|>assistant
TinyLlama-1.1B-Chat, i-34B-Chat, OpenHermes-2.5-Mistral-7B, Qwen, Dolphin-2.2-Yi-34B, Dolphin-2.6-Mistral-7B, Samantha-1.2-Mistral-7B, Orca-2-13B, Nous-Hermes-2-Mixtral-8x7B-DPO
openchat	
GPT4 User: {prompt}<|end_of_turn|>GPT4 Assistant:
OpenChat-3.5 series of models
zephyr	
<|system|>
{system_prompt}</s>
<|user|>
{prompt}</s>
<|assistant|>
Zephyr-7B-Alpha
codellama-instruct	
[INST] Write code to solve the following coding problem that obeys the constraints and passes the example test cases. Please wrap your code answer using ```:
{prompt}
[/INST]
Code-llama
mistral-instruct	
<s>[INST] {prompt} [/INST]
Mistral-7B-Instruct-v0.1, Mistral-7B-Instruct-v0.2, Mixtral-8x7B-Instruct-v0.1, miqu-2-70b
stabllm-zephyr	
<|user|>
{prompt}<|endoftext|>
<|assistant|>
stablelm-2-zephyr-1.6b
mistrallite	
<|prompter|>{prompt}</s><|assistant|>
MistralLite-7B
vicuna-1.0-chat	
{system} USER: {prompt} ASSISTANT:
Wizard-Vicuna-13B-Uncensored, Samantha-1.11-CodeLlama-34B, WizardLM-13B-V1.0-Uncensored
vicuna-1.1-chat	
USER: {prompt}
ASSISTANT:
CALM2-7B-Chat, Samantha-1.2-Mistral-7b
wizard-coder	
{system}

### Instruction:
{instruction}

### Response:
WizardCoder-Python-7B-V1.0
deepseek-chat	
User: {user_message_1}

Assistant: {assistant_message_1}<｜end▁of▁sentence｜>User: {user_message_2}

Assistant:
DeepSeek-LLM-7B-Chat
deepseek-coder	
{system}
### Instruction:
{question_1}
### Response:
{answer_1}
<|EOT|>
### Instruction:
{question_2}
### Response:
DeepSeek-Coder-6.7B
solar-instruct	
### User:
{prompt}

### Assistant:
SOLAR-10.7B-Instruct-v1.0
intel-neural	
### System:
{system}
### User:
{usr}
### Assistant:
Intel Nerual series of models
human-assistant	
Human: {input_1}\n\nAssistant:{output_1}Human: {input_2}\n\nAssistant: