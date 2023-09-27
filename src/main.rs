use parsemath::tokenizer::Tokenizer;

mod parsemath;

fn main() {
    //do something
    let my_token = Tokenizer::new("taco");
    println!("{:?}", my_token);
}
