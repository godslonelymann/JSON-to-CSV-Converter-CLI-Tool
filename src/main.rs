use std::env;
use std::path::Path;
use std::fs::{self, File};
use serde_json::Value;
use std::io::Write;

fn main() {
    println!("JSON to CSV Converter CLI Tool!");

    // Collect args
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <input.json> <output.csv>", args[0]);
        return;
    }

    let input_file = &args[1];
    let output_file = &args[2];

    let path = Path::new(input_file);

    if !path.is_file() {
        eprintln!("{}: does not exist (or is not a file)", input_file);
        return;
    }

    let data = fs::read_to_string(path).expect("Failed to read the file contents");
    let parsed_json: Value = serde_json::from_str(&data).expect("Invalid JSON file");

    // CASE 1: Array of objects
    if let Some(array) = parsed_json.as_array() {
        if array.is_empty() {
            println!("JSON array is empty!");
            return;
        }

        // Collect headers
        let mut headers: Vec<String> = Vec::new();
        for item in array {
            if let Some(obj) = item.as_object() {
                for (key, value) in obj {
                    if value.is_object() {
                        for (sub_key, _) in value.as_object().unwrap() {
                            let col_name = format!("{}.{}", key, sub_key);
                            if !headers.contains(&col_name) {
                                headers.push(col_name);
                            }
                        }
                    } else {
                        if !headers.contains(key) {
                            headers.push(key.clone());
                        }
                    }
                }
            }
        }

        let mut output = File::create(output_file).expect("Could not create output file");
        writeln!(output, "{}", headers.join(",")).expect("Failed to write headers");

        for item in array {
            if let Some(obj) = item.as_object() {
                let mut row: Vec<String> = Vec::new();

                for key in &headers {
                    if key.contains('.') {
                        let parts: Vec<&str> = key.split('.').collect();
                        let mut current = obj.get(parts[0]).unwrap_or(&Value::Null);

                        if let Some(sub_obj) = current.as_object() {
                            current = sub_obj.get(parts[1]).unwrap_or(&Value::Null);
                        } else if current.is_array() {
                            let arr_str: Vec<String> = current
                                .as_array()
                                .unwrap()
                                .iter()
                                .map(|v| {
                                    if let Some(s) = v.as_str() {
                                        s.to_string()
                                    } else {
                                        v.to_string()
                                    }
                                })
                                .collect();
                            row.push(format!("\"{}\"", arr_str.join(";").replace('"', "\"\"")));
                        }

                        if current.is_string() {
                            row.push(format!("\"{}\"", current.as_str().unwrap()));
                        } else if current.is_null() {
                            row.push("".to_string());
                        } else {
                            row.push(current.to_string());
                        }
                    } else {
                        let value = obj.get(key).unwrap_or(&Value::Null);

                        if value.is_string() {
                            row.push(format!("\"{}\"", value.as_str().unwrap()));
                        } else if value.is_array() {
                            let arr_str: Vec<String> = value
                                .as_array()
                                .unwrap()
                                .iter()
                                .map(|v| {
                                    if let Some(s) = v.as_str() {
                                        s.to_string()
                                    } else {
                                        v.to_string()
                                    }
                                })
                                .collect();
                            row.push(format!("\"{}\"", arr_str.join(";")));
                        } else if value.is_null() {
                            row.push("".to_string());
                        } else {
                            row.push(value.to_string());
                        }
                    }
                }

                writeln!(output, "{}", row.join(",")).expect("Failed to write row");
            }
        }

        println!("CSV file created: {}", output_file);
    }
    // CASE 2: Single object
    else if let Some(obj) = parsed_json.as_object() {
        println!("It is a single object!");

        let mut headers: Vec<String> = Vec::new();
        let mut row: Vec<String> = Vec::new();

        for (key, value) in obj {
            if value.is_object() {
                for (sub_key, sub_val) in value.as_object().unwrap() {
                    let col_name = format!("{}.{}", key, sub_key);
                    if !headers.contains(&col_name) {
                        headers.push(col_name.clone());
                    }

                    if sub_val.is_string() {
                        row.push(format!("\"{}\"", sub_val.as_str().unwrap()));
                    } else if sub_val.is_array() {
                        let arr_str: Vec<String> = sub_val
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|v| {
                                if let Some(s) = v.as_str() {
                                    s.to_string()
                                } else {
                                    v.to_string()
                                }
                            })
                            .collect();
                        row.push(format!("\"{}\"", arr_str.join(";")));
                    } else if sub_val.is_null() {
                        row.push("".to_string());
                    } else {
                        row.push(sub_val.to_string());
                    }
                }
            } else if value.is_array() {
                if !headers.contains(key) {
                    headers.push(key.clone());
                }
                let arr_str: Vec<String> = value
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| {
                        if let Some(s) = v.as_str() {
                            s.to_string()
                        } else {
                            v.to_string()
                        }
                    })
                    .collect();
                row.push(format!("\"{}\"", arr_str.join(";")));
            } else if value.is_string() {
                if !headers.contains(key) {
                    headers.push(key.clone());
                }
                row.push(format!("\"{}\"", value.as_str().unwrap()));
            } else if value.is_null() {
                if !headers.contains(key) {
                    headers.push(key.clone());
                }
                row.push("".to_string());
            } else {
                if !headers.contains(key) {
                    headers.push(key.clone());
                }
                row.push(value.to_string());
            }
        }

        let mut out = File::create(output_file).expect("Could not create output CSV file");
        writeln!(out, "{}", headers.join(",")).unwrap();
        writeln!(out, "{}", row.join(",")).unwrap();

        println!("CSV file created: {}", output_file);
    }
    // CASE 3: Unsupported JSON
    else {
        println!("JSON file not supported");
    }
}
