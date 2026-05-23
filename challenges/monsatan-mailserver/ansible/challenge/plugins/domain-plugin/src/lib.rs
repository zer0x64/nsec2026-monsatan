mod bindings {
    wit_bindgen::generate!({
        path: "wit/world.wit",
    });

    use super::DomainReplaceProcessor;

    export!(DomainReplaceProcessor);
}

struct DomainReplaceProcessor;

impl bindings::Guest for DomainReplaceProcessor {
    fn process(input: String) -> String {
        input.replace(
            "{domain}",
            &std::env::var("DOMAIN_NAME").unwrap_or_default(),
        )
    }
}
