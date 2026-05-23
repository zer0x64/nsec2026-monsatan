mod bindings {
    wit_bindgen::generate!({
        path: "wit/world.wit",
    });

    use super::UppercaseProcessor;

    export!(UppercaseProcessor);
}

struct UppercaseProcessor;

impl bindings::Guest for UppercaseProcessor {
    fn process(input: String) -> String {
        input.to_ascii_uppercase()
    }
}
