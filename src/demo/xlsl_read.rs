use calamine::{open_workbook_auto, DataType, Reader};
use serde::Serialize;
use serde_json;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write; // Bring io::Write into scope

#[derive(Serialize)]
struct ExcelData {
    // Define the structure of your data according to the Excel file
    // For example:
    col1: String,
    col2: String,
}

fn main() {
    // Open the Excel file
    if let Ok(mut excel) = open_workbook_auto("/home/wcs/workspace/t.xlsx") {
        if let Some(Ok(sheet)) = excel.worksheet_range("sheet") {
            // Prepare a vector to store the data
            let mut data: Vec<ExcelData> = Vec::new();

            for row in sheet.rows() {
                let col1 = match row.get(0) {
                    Some(DataType::String(s)) => s.to_string(),
                    _ => String::new(),
                };
                print!("col1: {}", col1);
                let col2 = match row.get(1) {
                    Some(DataType::Float(s)) => s.to_string(),
                    _ => String::new(),
                };

                // Create an instance of ExcelData and push it to the data vector
                data.push(ExcelData { col1, col2 });
            }

            // Convert the data to JSON
            let json_data = serde_json::to_string(&data).unwrap();

            // Write the JSON data to a file
            let file = File::create("/home/wcs/workspace/t.json").unwrap();
            // println!("file content: {}", json_data);
            let mut writer = BufWriter::new(file);
            // write file to disk path "/home/wcs/workspace/t.json"
            writer.write_all(json_data.as_bytes()).unwrap();
        }
    } else {
        println!("Failed to open file");
    }
}
