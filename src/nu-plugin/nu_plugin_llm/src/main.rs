use nu_plugin::{serve_plugin, LabeledError, Plugin, EvaluatedCall, MsgPackSerializer};
use nu_protocol::{Value, PluginSignature, Type};
use std::env;

struct LLM;

impl LLM {
    fn prompt(&self, call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
        // parse inputs
        // service enum Service("openai", "gemini", "local"), default to openai
        // model depends on Service, default to "gpt4-turbo" for openai
        // prompt is a string, required
        // temperature is a float, default to 0.1
        // max_tokens is an int, default to 1000. if flag "short" is set, default to 150. 
        //  if flag "medium" is set, default to 500. if flag "long" is set, default to 2000.

        let response = "dummy response for now";
        Ok(Value::String {
            val: format!("openai says: {}", response),
            internal_span: call.head,
        })
    }
}


impl Plugin for LLM {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![
            PluginSignature::build("prompt")
            .usage("time related greeting to the user")
            .input_output_type(Type::String, Type::String)
        ]
    }

    fn run(
        &mut self,
        name: &str,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        match name {
            "prompt" => self.prompt(call, input),
            _ => Err(LabeledError {
                label: "Unknown command".into(),
                msg: "Unknown command".into(),
                span: Some(call.head)
            }),
        }
    }
}

fn main() {
    serve_plugin(&mut LLM {}, MsgPackSerializer {})
}
