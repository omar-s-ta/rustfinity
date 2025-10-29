pub fn convert_temperature(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, String> {
    match (from_unit, to_unit) {
        ("C", "F") => Ok(value * (9.0 / 5.0) + 32.0),
        ("C", "K") => Ok(value + 273.15),
        ("F", "C") => Ok((value - 32.0) * (5.0 / 9.0)),
        ("F", "K") => {
            convert_temperature(value, "F", "C").and_then(|v| convert_temperature(v, "C", "K"))
        }
        ("K", "C") => Ok(value - 273.15),
        ("K", "F") => {
            convert_temperature(value, "K", "C").and_then(|v| convert_temperature(v, "C", "F"))
        }
        _ => Err("Invalid unit".to_string()),
    }
}
