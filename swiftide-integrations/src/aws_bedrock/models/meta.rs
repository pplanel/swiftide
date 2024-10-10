use serde::{Deserialize, Serialize};
const LLAMA_DEFAULT_SYSTEM_PROMPT: &str = "You are a helpful, respectful and honest assistant. Always answer as helpfully as possible, while being safe. Your answers should not include any harmful, unethical, racist, sexist, toxic, dangerous, or illegal content. Please ensure that your responses are socially unbiased and positive in nature.
\n\nIf a question does not make any sense, or is not factually coherent, explain why instead of answering something not correct. If you don't know the answer to a question, please don't share false information.";

#[derive(Serialize, Debug)]
pub(crate) struct LlammaRequest {
    pub(crate) max_tokens: i32, // differs per model
    pub(crate) prompt: String,

    // Optional fields
    #[serde(rename = "prompt")]
    pub(crate) system_prompt: Option<String>, // renamed to prompt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) max_gen_len: Option<i32>,

    // New field for images
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) images: Option<Vec<String>>, // added for images
}

// <|begin_of_text|><|start_header_id|>system<|end_header_id|>
//
// You are a helpful AI assistant for travel tips and recommendations<|eot_id|><|start_header_id|>user<|end_header_id|>
//
// What can you help me with?<|eot_id|><|start_header_id|>assistant<|end_header_id|>
impl LlammaRequest {
        pub(crate) fn generate_prompt(prompt: &str, system_prompt: Option<&str>) -> String {
            format!(
                "<s>[INST] <<SYS>>\n\
                {}\n\
                <</SYS>>\n\
                \n\
                {} [/INST]",
                 prompt,
                system_prompt.unwrap_or(LLAMA_DEFAULT_SYSTEM_PROMPT.into())
            )
        }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct LlammaResponse {
    pub(crate) generation: String, // added for generated text
    pub(crate) usage: LlammaUsage,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) stop_reason: Option<String>, // kept for reason why response stopped
}


#[derive(Serialize, Deserialize)]
pub(crate) struct LlammaUsage {
    pub(crate) prompt_token_count: i32, // added for number of tokens in the prompt
    pub(crate) generation_token_count: i32, // added for number of tokens in generated text
}
