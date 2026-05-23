mod bindings {
    wit_bindgen::generate!({
        path: "wit/world.wit",
    });

    use super::DbDumper;

    export!(DbDumper);
}

mod mysql;

use mysql::Connection;

struct DbDumper;

// MySQL connection details — adjust to match the target server.
const DB_HOST: &str = "[::1]:3306";
const DB_USER: &str = "root";
const DB_PASS: &str = "FLAG-{4f7146512805da9381cc7f49fa7242a9}";
const DB_NAME: &str = "mailserver";

const EGG: [u8; 256] = [0x41; 256];

impl bindings::Guest for DbDumper {
    fn process(input: String) -> String {
        // Executes the body as an SQL query and returns the results.
        let egg = format!("{:x?}\n\n", EGG);

        let mut conn = match Connection::connect(DB_HOST, DB_USER, DB_PASS, DB_NAME) {
            Ok(c) => c,
            Err(e) => return format!("Connection error: {e}"),
        };

        let output = match conn.query(&input) {
            Ok(rows) => rows.join("\n"),
            Err(e) => return format!("Query error: {e}"),
        };

        egg + "\n\n" + &output

        // Old code that just dumps the DB automatically
        // let sql1 = format!(
        //     "SELECT table_name FROM information_schema.tables where table_schema = 'mailserver';"
        // );

        // let mut conn = match Connection::connect(DB_HOST, DB_USER, DB_PASS, DB_NAME) {
        //     Ok(c) => c,
        //     Err(e) => return format!("Connection error: {e}"),
        // };

        // let tables = match conn.query(&sql1) {
        //     Ok(rows) => rows,
        //     Err(e) => return format!("Query error: {e}"),
        // };

        // buffer += &tables.join("\n");

        // buffer += "\n\n";

        // for t in tables[1..].into_iter() {
        //     let sql = format!("SELECT * FROM {}", t);

        //     let rows = match conn.query(&sql) {
        //         Ok(rows) => rows,
        //         Err(e) => return format!("Query error: {e}"),
        //     };
        //     buffer += &rows.join("\n");

        //     buffer += "\n\n";
        // }

        // buffer
    }
}
