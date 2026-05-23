mod bindings {
    wit_bindgen::generate!({
        path: "wit/world.wit",
    });

    use super::HelloWorldProcessor;

    export!(HelloWorldProcessor);
}

struct HelloWorldProcessor;

impl bindings::Guest for HelloWorldProcessor {
    fn process(_input: String) -> String {
        "Hello world!".to_string()
    }
}
