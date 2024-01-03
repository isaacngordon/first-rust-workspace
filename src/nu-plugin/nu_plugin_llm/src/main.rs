use nu_plugin::{serve_plugin, EvaluatedCall, LabeledError, MsgPackSerializer, Plugin};
use nu_protocol::{PluginSignature, Type, Value};

use core::llm::openai;

struct LLM;

impl LLM {
    fn print_message(msg: &openai::Message) {
        let emoji = match msg.role.as_str() {
            "assistant" => "🤖",
            "user" => "👤",
            "system" => "🖥️",
            _ => "👽",
        };
        println!("{} {} says: {}", emoji, msg.role, msg.content)
    }

    fn prompt(&self, call: &EvaluatedCall) -> Result<Value, LabeledError> {
        // parse inputs
        // service enum Service("openai", "gemini", "local"), default to openai
        // model depends on Service, default to "gpt4-turbo" for openai
        // prompt is a string, required
        // temperature is a float, default to 0.1
        // max_tokens is an int, default to 1000. if flag "short" is set, default to 150.
        //  if flag "medium" is set, default to 500. if flag "long" is set, default to 2000.

        let user_msg: String = call.req(0)?;
        let conversation = openai::Conversation { messages: vec![] };

        Self::print_message(&openai::Message {
            role: "user".to_string(),
            content: user_msg.clone(),
        });

        // turning an async function into a blocking function
        let response = futures::executor::block_on(
            openai::prompt(user_msg, conversation)
        );

        match response {
            Ok(msg) => {
                Self::print_message(&msg);
                Ok(Value::String { val: msg.content, internal_span: call.head })
            },
            Err(error) => Err(LabeledError {
                label: "Error".into(),
                msg: format!("Error: {}", error),
                span: Some(call.head),
            }),
        }
    }
}

impl Plugin for LLM {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![PluginSignature::build("prompt")
            .usage("text prompt")
            .input_output_type(Type::String, Type::String)]
    }

    fn run(
        &mut self,
        name: &str,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        match name {
            "prompt" => {
                let result = self.prompt(call);
                result
            },
            _ => Err(LabeledError {
                label: "Unknown command".into(),
                msg: "Unknown command".into(),
                span: Some(call.head),
            }),
        }
    }
}

fn main() {
    serve_plugin(&mut LLM {}, MsgPackSerializer {})
}
