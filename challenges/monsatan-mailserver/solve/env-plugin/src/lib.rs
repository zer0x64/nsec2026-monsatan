mod bindings {
    wit_bindgen::generate!({
        path: "wit/world.wit",
    });

    use super::EnvDumper;

    export!(EnvDumper);
}

struct EnvDumper;

const EGG: [u8; 256] = [0x41; 256];

impl bindings::Guest for EnvDumper {
    fn process(_input: String) -> String {
        let mut buffer = format!("{:?}\n", &EGG);
        for (k, v) in std::env::vars() {
            buffer += &format!("{}={}\n", k, v);
        }

        buffer
    }
}
