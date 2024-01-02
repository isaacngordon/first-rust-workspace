use nu_plugin::{serve_plugin, LabeledError, Plugin, EvaluatedCall, MsgPackSerializer};
use nu_protocol::{Value, PluginSignature, Type};
use std::env;

struct LLM;

impl LLM {
    fn prompt(&self, call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
        // Add your code here
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
