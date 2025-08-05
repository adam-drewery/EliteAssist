use crate::state::State;
use iced::futures::Stream;
use iced::Subscription;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use crate::gui::Message;

pub fn subscription(_state: &State) -> Subscription<Message> {
    Subscription::run(stream_events)
}

#[cfg(not(feature = "mock_events"))]
fn stream_events() -> impl Stream<Item=Message> {
    use crate::journal::JournalWatcher;

    let (sender, receiver) = mpsc::channel(16);

    tokio::spawn(async move {
        let mut watcher = JournalWatcher::new();
        loop {
            let input = watcher.next().await;
            sender.send(Message::JournalEvent(input)).await.unwrap();
        }
    });

    ReceiverStream::new(receiver)
}

#[cfg(feature = "mock_events")]
fn stream_events() -> impl Stream<Item = Message> {
    use tokio::fs;
    use std::path::PathBuf;
    use crate::event::JournalEvent;

    let (sender, receiver) = mpsc::channel(16);

    tokio::spawn(async move {
        let example_dir = PathBuf::from("src/example_data");
        let mut files = Vec::new();
        let mut dirs = vec![example_dir];

        while let Some(dir) = dirs.pop() {
            let mut entries = fs::read_dir(dir).await.unwrap();
            while let Ok(Some(entry)) = entries.next_entry().await {
                let path = entry.path();
                if path.is_dir() {
                    dirs.push(path);
                } else if path.extension().map_or(false, |ext| ext == "json") {
                    files.push(path);
                }
            }
        }

        for file in files {
            let content = fs::read_to_string(file).await.unwrap();
            let mut events: Vec<JournalEvent> = serde_json::from_str(&content).unwrap();
            use rand::seq::SliceRandom;
            events.shuffle(&mut rand::rng());
            for event in events {
                sender.send(Message::JournalEvent(event)).await.unwrap();
            }
        }
    });

    ReceiverStream::new(receiver)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::event::JournalEvent;
    use regex::Regex;
    use std::fs;
    use serde_json::Value;

    fn test_deserialize_file(path: &PathBuf) -> Result<(), String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;

        serde_json::from_str::<Vec<JournalEvent>>(&content).map_err(|e| {
            let error = format!("Failed to deserialize {}: {}", path.display(), e);
            let re = Regex::new(r"(.*?), expected one of.*?(at line \d+ column \d+)").unwrap();
            if let Some(captures) = re.captures(&error) {
                format!("{} {}", &captures[1], &captures[2])
            } else {
                error
            }
        })?;
        Ok(())
    }

    #[test]
    fn test_example_files_deserialization() {
        let example_dir = PathBuf::from("src/example_data");
        let mut files = Vec::new();
        let mut dirs = vec![example_dir];

        while let Some(dir) = dirs.pop() {
            for entry in fs::read_dir(dir).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_dir() {
                    dirs.push(path);
                } else if path.extension().map_or(false, |ext| ext == "json") {
                    files.push(path);
                }
            }
        }

        let mut failed = Vec::new();
        let mut missing_variants = Vec::new();
        let variant_re = Regex::new(r"unknown variant `(\w+)`").unwrap();

        for file in files {
            if let Err(error) = test_deserialize_file(&file) {
                if let Some(captures) = variant_re.captures(&error) {
                    missing_variants.push(captures[1].to_string());
                }
                failed.push(error);
            }
        }

        assert!(
            failed.is_empty(),
            "Failed to deserialize the following files:\n{}\n\nMissing enum variants:\n{}",
            failed.join("\n"),
            missing_variants.join("\n")
        );
    }
    
    /// Fetches a JSON schema from the specified file path
    ///
    /// This function reads a JSON schema from a local file path and parses it as JSON.
    /// It returns the parsed JSON schema or an error message if the file read fails.
    fn fetch_schema(file_path: &str) -> Result<Value, String> {
        // Read the schema from the local file
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read schema file {}: {}", file_path, e))?;
        
        // Parse the JSON content
        serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse schema from {}: {}", file_path, e))
    }

    /// Validates an event against its JSON schema
    ///
    /// This function fetches the event-specific schema and the base _Event.json schema,
    /// then checks if the event has all the required properties defined in both schemas.
    /// It returns Ok(()) if the event is valid, or an error message listing the missing properties.
    fn validate_event_with_schema(event: &Value, event_name: &str) -> Result<(), String> {
        // Base path for schemas
        let base_path = "ed-journal-schemas/schemas";
        
        // File paths for the schemas
        let event_schema_path = format!("{}/{}/{}.json", base_path, event_name, event_name);
        let base_schema_path = format!("{}/{}", base_path, "_Event.json");
        
        // Fetch the event-specific schema
        let event_schema = fetch_schema(&event_schema_path)
            .map_err(|e| format!("Failed to fetch schema for {}: {}", event_name, e))?;
            
        // Fetch the base _Event.json schema
        let base_schema = fetch_schema(&base_schema_path)
            .map_err(|e| format!("Failed to fetch base schema: {}", e))?;
        
        // Simplified validation: check if the event has all required properties
        let mut missing_properties = Vec::new();
        
        // Check base schema required properties
        if let Value::Object(base) = &base_schema {
            if let Some(Value::Array(required)) = base.get("required") {
                for req in required {
                    if let Value::String(prop_name) = req {
                        if !event.get(prop_name).is_some() {
                            missing_properties.push(format!("Missing base property: {}", prop_name));
                        }
                    }
                }
            }
        }
        
        // Check event schema required properties
        if let Value::Object(schema) = &event_schema {
            if let Some(Value::Array(required)) = schema.get("required") {
                for req in required {
                    if let Value::String(prop_name) = req {
                        if !event.get(prop_name).is_some() {
                            missing_properties.push(format!("Missing event property: {}", prop_name));
                        }
                    }
                }
            }
        }
        
        // Report any missing properties
        if !missing_properties.is_empty() {
            return Err(format!("\n{}", missing_properties.join("\n")));
        }
        
        Ok(())
    }
    
    /// Tests that example files in the example_data directory are valid according to their JSON schemas
    ///
    /// This test:
    /// 1. Recursively finds all JSON files in the example_data directory
    /// 2. For each file, extracts the event name from each event
    /// 3. Fetches the corresponding JSON schema from the GitHub repository
    /// 4. Validates the event against the schema
    /// 5. Reports validation results, including:
    ///    - Successfully validated events
    ///    - Events with missing schemas
    ///    - Events with validation errors (missing required properties)
    ///
    /// The test provides a detailed report but does not fail if there are validation errors,
    /// as the purpose is to identify which examples need to be updated to match their schemas.
    #[test]
    fn test_example_files_schema_validation() {
        let example_dir = PathBuf::from("src/example_data");
        let mut files = Vec::new();
        let mut dirs = vec![example_dir];

        // Find all JSON files in example_data directory
        while let Some(dir) = dirs.pop() {
            for entry in fs::read_dir(dir).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_dir() {
                    dirs.push(path);
                } else if path.extension().map_or(false, |ext| ext == "json") {
                    files.push(path);
                }
            }
        }

        let mut validation_failures = Vec::new();
        let mut schema_not_found = Vec::new();
        let mut other_errors = Vec::new();
        let mut validated_count = 0;

        for file in files {
            // Read and parse the file
            let content = match fs::read_to_string(&file) {
                Ok(content) => content,
                Err(e) => {
                    other_errors.push(format!("Failed to read {}: {}", file.display(), e));
                    continue;
                }
            };
            
            let events: Vec<Value> = match serde_json::from_str(&content) {
                Ok(events) => events,
                Err(e) => {
                    other_errors.push(format!("Failed to parse {}: {}", file.display(), e));
                    continue;
                }
            };
            
            if events.is_empty() {
                other_errors.push(format!("No events found in {}", file.display()));
                continue;
            }
            
            // Process each event in the file
            for event in &events {
                // Extract the event name
                let event_name = match event.get("event") {
                    Some(Value::String(name)) => name,
                    _ => {
                        other_errors.push(format!("Missing or invalid 'event' field in {}", file.display()));
                        continue;
                    }
                };
                
                // Validate the event against its schema
                match validate_event_with_schema(event, event_name) {
                    Ok(_) => {
                        validated_count += 1;
                        println!("{}: OK", event_name);
                    }
                    Err(error) => {
                        if error.contains("HTTP status 404") {
                            schema_not_found.push(format!("{} (in {})", event_name, file.display()));
                        } else {
                            validation_failures.push(format!("{} in {}: {}", event_name, file.display(), error));
                        }
                    }
                }
            }
        }

        // Print summary information
        println!("Schema validation summary:");
        println!("  Successfully validated: {} events", validated_count);
        println!("  Schemas not found: {} events", schema_not_found.len());
        println!("  Validation failures: {} events", validation_failures.len());
        
        if !schema_not_found.is_empty() {
            println!("\nSchemas not found for the following events:");
            for event in &schema_not_found {
                println!("  {}", event);
            }
        }
        
        if !validation_failures.is_empty() {
            println!("\nValidation failures (events missing required properties):");
            for failure in &validation_failures {
                println!("  {}", failure);
            }
            
            // Fail the test if there are validation failures
            assert!(
                validation_failures.is_empty(),
                "Found {} validation failures. Fix the example files to match their schemas.",
                validation_failures.len(),
            );
        }
        
        if !other_errors.is_empty() {
            println!("\nOther errors:");
            for error in &other_errors {
                println!("  {}", error);
            }
            
            // Fail the test if there are errors reading or parsing files
            assert!(
                false,
                "Encountered errors reading or parsing files:\n{}",
                other_errors.join("\n")
            );
        }
        
        // Test now fails if there are validation failures or other errors
    }
}
