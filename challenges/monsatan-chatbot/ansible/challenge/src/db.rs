use std::sync::{Arc, Mutex};

use rusqlite::{params, types::Value, Connection, Result};

const SEED_DATA: &[(&str, &str, f64)] = &[
    ("SolarBurst™ Helios Array I", "Solar", 450.0),
    ("SolarBurst™ Helios Array II", "Solar", 480.0),
    ("WindRise™ Prairie Turbine Farm", "Wind", 320.0),
    ("AquaPower™ Cascade Station", "Hydroelectric", 800.0),
    ("OmniPower™ Fusion Pilot", "Nuclear Fusion", 2000.0),
    ("EverGreen™ Geothermal Hub", "Geothermal", 200.0),
    ("SafeGro™ BioGas Digester Plant", "Biomass", 150.0),
    ("SafeGro™ BioGas Digester Plant 2", "Biomass", 170.0),
    (
        "GreenTire™ Thermal Station - FLAG-{085a1ccb46afb053063f2e73dce5f298}",
        "Tire Combustion",
        12000.0,
    ),
];

/// Opens (or creates) the SQLite database at `path`, creates the schema if
/// needed, and seeds it with the initial power plant data.
pub fn init(path: &str) -> Result<Arc<Mutex<Connection>>> {
    let conn = Connection::open(path)?;

    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS power_plants (
            id               INTEGER PRIMARY KEY AUTOINCREMENT,
            name             TEXT    NOT NULL,
            type             TEXT    NOT NULL,
            power_output_mw  REAL    NOT NULL
        );",
    )?;

    // Only seed on first boot (empty table).
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM power_plants", [], |row| row.get(0))?;

    if count == 0 {
        seed(&conn)?;
    }

    Ok(Arc::new(Mutex::new(conn)))
}

/// Inserts the initial power plant rows.
fn seed(conn: &Connection) -> Result<()> {
    for (name, plant_type, output) in SEED_DATA {
        conn.execute(
            "INSERT INTO power_plants (name, type, power_output_mw)
             VALUES (?1, ?2, ?3)",
            params![name, plant_type, output],
        )?;
    }
    Ok(())
}

/// Executes a SQL query against the database and returns the result as a
/// plain-text table.  Only SELECT statements are accepted; anything else is
/// rejected with an error message.
///
/// Note: the SELECT check is intentionally minimal — Monsatan's internal
/// tooling team wrote it on a Friday afternoon.
pub fn execute_select(db: &Arc<Mutex<Connection>>, query: &str) -> Result<String, String> {
    // Basic guard: only allow statements that start with SELECT.
    if !query.trim().to_lowercase().starts_with("select") {
        return Err("Only SELECT queries are permitted.".to_string());
    }

    let conn = db.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn.prepare(query).map_err(|e| e.to_string())?;

    // Collect column names for the header row.
    let col_names: Vec<String> = stmt.column_names().into_iter().map(str::to_owned).collect();

    // Collect all rows as strings.
    let rows: Vec<Vec<String>> = stmt
        .query_map([], |row| {
            let values: Vec<String> = (0..col_names.len())
                .map(|i| match row.get::<_, Value>(i) {
                    Ok(Value::Null) => "NULL".to_string(),
                    Ok(Value::Integer(n)) => n.to_string(),
                    Ok(Value::Real(f)) => f.to_string(),
                    Ok(Value::Text(s)) => s,
                    Ok(Value::Blob(b)) => format!("<blob {} bytes>", b.len()),
                    Err(_) => "ERROR".to_string(),
                })
                .collect();
            Ok(values)
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<_, _>>()
        .map_err(|e| e.to_string())?;

    if rows.is_empty() {
        return Ok(format!("{}\n(no rows)", col_names.join(" | ")));
    }

    // Format as a simple pipe-delimited table.
    let mut output = col_names.join(" | ");
    output.push('\n');
    for row in rows {
        output.push_str(&row.join(" | "));
        output.push('\n');
    }

    Ok(output)
}
