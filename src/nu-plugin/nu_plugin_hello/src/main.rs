use nu_plugin::{serve_plugin, LabeledError, Plugin, EvaluatedCall, MsgPackSerializer};
use nu_protocol::{Value, PluginSignature, Type};
use std::env;
use chrono::Timelike;

enum TimeOfDay {
    Morning,
    Afternoon,
    Evening,
}

struct Hello;

impl Hello {
    fn greet(&self, call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
        let username = env::var("USER").or_else(|_| env::var("USERNAME"));
        let user = match username {
            Ok(name) => name,
            Err(_) => {
                eprintln!("Cannot determine the current user");
                "there".to_string()
            },
        };
        
        let time_of_day = match chrono::Local::now().hour() {
            0..=11 => TimeOfDay::Morning,
            12..=17 => TimeOfDay::Afternoon,
            _ => TimeOfDay::Evening,
        };
    
        let greeting = match time_of_day {
            TimeOfDay::Morning => "Good morning",
            TimeOfDay::Afternoon => "Good afternoon",
            TimeOfDay::Evening => "Good evening",
        };

        let greetee = match input {
            Value::String { val, .. } => val.clone(),
            _ => user,
        };

        Ok(Value::String {
            val: format!("{}, {}", greeting, greetee),
            internal_span: call.head,
        })
    }
}


impl Plugin for Hello {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![PluginSignature::build("hello")
            .usage("time related greeting to the user")
            .input_output_type(Type::Nothing, Type::String)
        ]
    }

    fn run(
        &mut self,
        name: &str,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        match name {
            "hello" => self.greet(call, input),
            "goodbye" => Ok(Value::String {
                val: format!("Goodbye, {}!", input.as_string()?),
                internal_span: call.head,
            }),
            _ => Err(LabeledError {
                label: "Unknown command".into(),
                msg: "Unknown command".into(),
                span: Some(call.head)
            }),
        }
    }
}

fn main() {
    serve_plugin(&mut Hello {}, MsgPackSerializer {})
}
