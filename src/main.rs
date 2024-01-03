mod core;
fn main(){
    println!("Hello, world!");
    core::llm::openai::call();
    core::my_shell::run();
}