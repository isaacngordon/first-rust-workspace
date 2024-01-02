/**
 * This is an example of a plugin that can be used in nushell, with several commands. 
 * Code is a modified version of the example plugin in the nushell repository (see source).
 * 
 * Source: https://github.com/nushell/nushell/tree/main/crates/nu_plugin_example 
 */

use nu_plugin::{EvaluatedCall, LabeledError, Plugin, serve_plugin, MsgPackSerializer};
use nu_protocol::{Record, Value, PluginSignature, SyntaxShape, PluginExample, Category};

pub struct Example;

impl Example {
    fn print_values(
        &self,
        index: u32,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<(), LabeledError> {
        // Note. When debugging your plugin, you may want to print something to the console
        // Use the eprintln macro to print your messages. Trying to print to stdout will
        // cause a decoding error for your message
        eprintln!("Calling test {index} signature");
        eprintln!("value received {input:?}");

        // To extract the arguments from the Call object you can use the functions req, has_flag,
        // opt, rest, and get_flag
        //
        // Note that plugin calls only accept simple arguments, this means that you can
        // pass to the plug in Int and String. This should be improved when the plugin has
        // the ability to call back to NuShell to extract more information
        // Keep this in mind when designing your plugin signatures
        let a: i64 = call.req(0)?;
        let b: String = call.req(1)?;
        let flag = call.has_flag("flag");
        let opt: Option<i64> = call.opt(2)?;
        let named: Option<String> = call.get_flag("named")?;
        let rest: Vec<String> = call.rest(3)?;

        eprintln!("Required values");
        eprintln!("a: {a:}");
        eprintln!("b: {b:}");
        eprintln!("flag: {flag:}");
        eprintln!("rest: {rest:?}");

        if let Some(v) = opt {
            eprintln!("Found optional value opt: {v:}")
        } else {
            eprintln!("No optional value found")
        }

        if let Some(v) = named {
            eprintln!("Named value: {v:?}")
        } else {
            eprintln!("No named value found")
        }

        Ok(())
    }

    pub fn test1(&self, call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
        self.print_values(1, call, input)?;

        Ok(Value::nothing(call.head))
    }

    pub fn test2(&self, call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
        self.print_values(2, call, input)?;

        let cols = vec!["one".to_string(), "two".to_string(), "three".to_string()];

        let vals = (0..10i64)
            .map(|i| {
                let vals = (0..3)
                    .map(|v| Value::int(v * i, call.head))
                    .collect::<Vec<Value>>();

                Value::record(Record::from_raw_cols_vals(cols.clone(), vals), call.head)
            })
            .collect::<Vec<Value>>();

        Ok(Value::list(vals, call.head))
    }

    pub fn test3(&self, call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
        self.print_values(3, call, input)?;

        Err(LabeledError {
            label: "ERROR from plugin".into(),
            msg: "error message pointing to call head span".into(),
            span: Some(call.head),
        })
    }
}

impl Plugin for Example {
    fn signature(&self) -> Vec<PluginSignature> {
        // It is possible to declare multiple signature in a plugin
        // Each signature will be converted to a command declaration once the
        // plugin is registered to nushell
        vec![
            PluginSignature::build("nu-example-1")
                .usage("PluginSignature test 1 for plugin. Returns Value::Nothing")
                .extra_usage("Extra usage for nu-example-1")
                .search_terms(vec!["example".into()])
                .required("a", SyntaxShape::Int, "required integer value")
                .required("b", SyntaxShape::String, "required string value")
                .switch("flag", "a flag for the signature", Some('f'))
                .optional("opt", SyntaxShape::Int, "Optional number")
                .named("named", SyntaxShape::String, "named string", Some('n'))
                .rest("rest", SyntaxShape::String, "rest value string")
                .plugin_examples(vec![PluginExample {
                    example: "nu-example-1 3 bb".into(),
                    description: "running example with an int value and string value".into(),
                    result: None,
                }])
                .category(Category::Experimental),
            PluginSignature::build("nu-example-2")
                .usage("PluginSignature test 2 for plugin. Returns list of records")
                .required("a", SyntaxShape::Int, "required integer value")
                .required("b", SyntaxShape::String, "required string value")
                .switch("flag", "a flag for the signature", Some('f'))
                .optional("opt", SyntaxShape::Int, "Optional number")
                .named("named", SyntaxShape::String, "named string", Some('n'))
                .rest("rest", SyntaxShape::String, "rest value string")
                .category(Category::Experimental),
            PluginSignature::build("nu-example-3")
                .usage("PluginSignature test 3 for plugin. Returns labeled error")
                .required("a", SyntaxShape::Int, "required integer value")
                .required("b", SyntaxShape::String, "required string value")
                .switch("flag", "a flag for the signature", Some('f'))
                .optional("opt", SyntaxShape::Int, "Optional number")
                .named("named", SyntaxShape::String, "named string", Some('n'))
                .rest("rest", SyntaxShape::String, "rest value string")
                .category(Category::Experimental),
        ]
    }

    fn run(
        &mut self,
        name: &str,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        // You can use the name to identify what plugin signature was called
        match name {
            "nu-example-1" => self.test1(call, input),
            "nu-example-2" => self.test2(call, input),
            "nu-example-3" => self.test3(call, input),
            _ => Err(LabeledError {
                label: "Plugin call with wrong name signature".into(),
                msg: "the signature used to call the plugin does not match any name in the plugin signature vector".into(),
                span: Some(call.head),
            }),
        }
    }
}

fn main() {
    // When defining your plugin, you can select the Serializer that could be
    // used to encode and decode the messages. The available options are
    // MsgPackSerializer and JsonSerializer. Both are defined in the serializer
    // folder in nu-plugin.
    serve_plugin(&mut Example {}, MsgPackSerializer {})

    // Note
    // When creating plugins in other languages one needs to consider how a plugin
    // is added and used in nushell.
    // The steps are:
    // - The plugin is register. In this stage nushell calls the binary file of
    //      the plugin sending information using the encoded PluginCall::PluginSignature object.
    //      Use this encoded data in your plugin to design the logic that will return
    //      the encoded signatures.
    //      Nushell is expecting and encoded PluginResponse::PluginSignature with all the
    //      plugin signatures
    // - When calling the plugin, nushell sends to the binary file the encoded
    //      PluginCall::CallInfo which has all the call information, such as the
    //      values of the arguments, the name of the signature called and the input
    //      from the pipeline.
    //      Use this data to design your plugin login and to create the value that
    //      will be sent to nushell
    //      Nushell expects an encoded PluginResponse::Value from the plugin
    // - If an error needs to be sent back to nushell, one can encode PluginResponse::Error.
    //      This is a labeled error that nushell can format for pretty printing
}