mod bindings {
    wit_bindgen::generate!({
        path: "wit/world.wit",
    });

    use super::QuoteProcessor;

    export!(QuoteProcessor);
}

struct QuoteProcessor;

impl bindings::Guest for QuoteProcessor {
    fn process(input: String) -> String {
        let quote = ureq::get("http://[::1]:3001/quote")
            .call()
            .unwrap()
            .body_mut()
            .read_to_string()
            .unwrap();

        input.replace("{quote}", quote.as_str())
    }
}
