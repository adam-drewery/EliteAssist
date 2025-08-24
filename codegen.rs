pub mod util {
    use std::path::Path;

    pub fn rust_string_lit(s: &str) -> String {
        let mut out = String::with_capacity(s.len() + 2);
        out.push('"');
        for ch in s.chars() {
            match ch {
                '\\' => out.push_str("\\\\"),
                '"' => out.push_str("\\\""),
                '\n' => out.push_str("\\n"),
                '\r' => out.push_str("\\r"),
                '\t' => out.push_str("\\t"),
                _ => out.push(ch),
            }
        }
        out.push('"');
        out
    }

    pub fn sanitize_field(header: &str) -> String {
        let s = header.trim().to_lowercase();
        if s == "type" {
            return "r#type".to_string();
        }
        let mut result = String::new();
        for (i, ch) in s.chars().enumerate() {
            let valid = ch.is_ascii_alphanumeric() || ch == '_';
            let ch = if valid { ch } else { '_' };
            if i == 0 && ch.is_ascii_digit() {
                result.push('_');
            }
            result.push(ch);
        }
        if result.is_empty() { "_".to_string() } else { result }
    }

    pub fn load_csv(path: &Path) -> anyhow::Result<(Vec<String>, Vec<Vec<String>>)> {
        let mut rdr = csv::Reader::from_path(path)?;
        let headers = rdr
            .headers()?
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let mut rows = Vec::new();
        for result in rdr.records() {
            let rec = result?;
            rows.push(rec.iter().map(|s| s.to_string()).collect::<Vec<_>>());
        }
        Ok((headers, rows))
    }
}

pub mod fdev_ids;
pub mod inara;
pub mod events;
