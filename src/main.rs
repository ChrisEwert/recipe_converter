use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;
use serde::Deserialize;

enum MyError {
    FileCreateError(String),
    TextInsertError(String, String),
    UserInputError,
    FileNotFoundError,
    NotAJsonFileError,
    FileCopyError(String),
    JsonParsingError(String),
    FileOpenError(String),
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct Recipe {
    name: String,
    author: String,
    dateOfCreation: String,
    ingredients: Vec<String>,
    content: Vec<String>,
    categories: Vec<String>,
    cookingTimeInMinutes: u32,
}

fn create_new_file(directory: &str, file_name: &str) -> Result<(), MyError> {
    std::fs::create_dir_all(directory)
        .map_err(|_| MyError::FileCreateError(directory.to_string()))?;

    let file_path = format!("{}/{}", directory, file_name);
    File::create(file_path)
        .map_err(|_| MyError::FileCreateError(file_name.to_string()))?;

    Ok(())
}

fn get_json_file_path() -> Result<String, MyError> {
    println!("Please enter the path to a JSON file:");
    let mut input = String::new();

    io::stdin()
        .lock()
        .read_line(&mut input)
        .map_err(|_| MyError::UserInputError)?;

    let path = input.trim().to_string();
    if !Path::new(&path).exists() {
        return Err(MyError::FileNotFoundError);
    }

    let extension = Path::new(&path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");
    if extension.to_lowercase() != "json" {
        return Err(MyError::NotAJsonFileError);
    }

    Ok(path)
}

fn put_recipes_from_path_into_md_file(path: &String, directory: &str, file_name_md: &str) -> Result<(), MyError> {
    let source_file = File::open(&path).map_err(|_| MyError::FileCopyError(path.clone()))?;
    let recipes: std::collections::BTreeMap<String, Recipe> = serde_json::from_reader(source_file)
        .map_err(|_| MyError::JsonParsingError(path.clone()))?;

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(format!("{}/{}", directory, file_name_md))
        .map_err(|_| MyError::FileOpenError(file_name_md.to_string()))?;

    write_line(&mut file, "# Cookbook", file_name_md)?;
    write_line(&mut file, "", file_name_md)?;

    let mut toc_links = Vec::new();
    for (_, recipe) in &recipes {
        toc_links.push(format!("* [{} by {}](#{})", recipe.name.to_uppercase(), recipe.author, replace_spaces_with_dashes(recipe.name.to_lowercase())));
    }

    write_line(&mut file, "## Table of Contents", file_name_md)?;
    write_line(&mut file, "", file_name_md)?;
    for link in &toc_links {
        write_line(&mut file, link, file_name_md)?;
    }
    write_line(&mut file, "", file_name_md)?;

    for (_, recipe) in &recipes {
        write_line(&mut file, &format!("## {}", recipe.name), file_name_md)?;
        write_line(&mut file, "", file_name_md)?;

        write_line(&mut file, "### Attributes", file_name_md)?;
        write_line(&mut file, "", file_name_md)?;

        write_line(&mut file, &format!("* **Author**: {}", recipe.author), file_name_md)?;
        write_line(&mut file, &format!("* **Creation date**: {}", convert_date_format(&recipe.dateOfCreation)), file_name_md)?;
        write_line(&mut file, &format!("* **Categories**: {}", recipe.categories.join(", ")), file_name_md)?;
        write_line(&mut file, &format!("* **Cooking time**: {}", format_minutes(recipe.cookingTimeInMinutes)), file_name_md)?;
        write_line(&mut file, "", file_name_md)?;

        write_line(&mut file, "### Ingredients", file_name_md)?;
        write_line(&mut file, "", file_name_md)?;

        for ingredient in &recipe.ingredients {
            write_line(&mut file, &format!("* {}", ingredient), file_name_md)?;
        }
        write_line(&mut file, "", file_name_md)?;

        write_line(&mut file, "### Cooking steps", file_name_md)?;
        write_line(&mut file, "", file_name_md)?;
        
        for (i, step) in recipe.content.iter().enumerate() {
            write_line(&mut file, &format!("* Step {}", i + 1), file_name_md)?;
            write_line(&mut file, &format!("    * {}", step), file_name_md)?;
        }
        
        write_line(&mut file, "", file_name_md)?;
    }

    Ok(())
}

fn put_recipes_from_path_into_adoc_file(path: &String, directory: &str, file_name_adoc: &str) -> Result<(), MyError> {
    let source_file = File::open(&path).map_err(|_| MyError::FileCopyError(path.clone()))?;
    let recipes: std::collections::BTreeMap<String, Recipe> = serde_json::from_reader(source_file)
        .map_err(|_| MyError::JsonParsingError(path.clone()))?;

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(format!("{}/{}", directory, file_name_adoc))
        .map_err(|_| MyError::FileOpenError(file_name_adoc.to_string()))?;

    write_line(&mut file, "= Cookbook", file_name_adoc)?;

    write_line(&mut file, ":toc:", file_name_adoc)?;
    write_line(&mut file, ":toclevels: 1", file_name_adoc)?;
    write_line(&mut file, "", file_name_adoc)?;

    for (_, recipe) in &recipes {
        write_line(&mut file, &format!("== {}", recipe.name), file_name_adoc)?;
        write_line(&mut file, "", file_name_adoc)?;

        write_line(&mut file, "=== Attributes", file_name_adoc)?;
        write_line(&mut file, "", file_name_adoc)?;

        write_line(&mut file, &format!("* **Author**: {}", recipe.author), file_name_adoc)?;
        write_line(&mut file, &format!("* **Creation date**: {}", convert_date_format(&recipe.dateOfCreation)), file_name_adoc)?;
        write_line(&mut file, &format!("* **Categories**: {}", recipe.categories.join(", ")), file_name_adoc)?;
        write_line(&mut file, &format!("* **Cooking time**: {}", format_minutes(recipe.cookingTimeInMinutes)), file_name_adoc)?;
        write_line(&mut file, "", file_name_adoc)?;

        write_line(&mut file, "=== Ingredients", file_name_adoc)?;
        write_line(&mut file, "", file_name_adoc)?;

        for ingredient in &recipe.ingredients {
            write_line(&mut file, &format!("* {}", ingredient), file_name_adoc)?;
        }
        write_line(&mut file, "", file_name_adoc)?;

        write_line(&mut file, "=== Cooking steps", file_name_adoc)?;
        write_line(&mut file, "", file_name_adoc)?;
        
        for (i, step) in recipe.content.iter().enumerate() {
            write_line(&mut file, &format!("* Step {}", i + 1), file_name_adoc)?;
            write_line(&mut file, &format!("** {}", step), file_name_adoc)?;
        }
        
        write_line(&mut file, "", file_name_adoc)?;
    }

    Ok(())
}

fn write_line(file: &mut File, content: &str, file_name_md: &str) -> Result<(), MyError> {
    write!(file, "{}", content)
        .map_err(|_| MyError::TextInsertError(file_name_md.to_string(), content.to_string()))?;

    writeln!(file)
    .map_err(|_| MyError::TextInsertError("".to_string(), content.to_string()))?;

    Ok(())
}

fn replace_spaces_with_dashes(input: String) -> String {
    input.replace(" ", "-")
}

fn convert_date_format(date_str: &str) -> String {
    let parts: Vec<&str> = date_str.split('-').collect();
    if parts.len() != 3 {
        return "".to_string();
    }

    let day = parts[2];
    let month = parts[1];
    let year = parts[0];

    format!("{}.{}.{}", day, month, year)
}

fn format_minutes(minutes: u32) -> String {
    if minutes >= 60 {
        let hours = minutes / 60;
        let minutes = minutes % 60;
        format!("{:02}:{:02}h", hours, minutes)
    } else {
        format!("{}min", minutes)
    }
}


fn main() {
    let directory = "result";
    let file_name_md = "cookbook.md";
    let file_name_adoc = "cookbook.adoc";

    println!("");
    if let Err(_) = create_new_file(directory, file_name_md) {
        eprintln!("Error while trying to create file {}!", file_name_md);
        return;
    }
    println!("Successfully created md file!");

    if let Err(_) = create_new_file(directory, file_name_adoc) {
        eprintln!("Error while trying to create file {}!", file_name_adoc);
        return;
    }
    println!("Successfully created adoc file!");

    println!("");
    let path = match get_json_file_path() {
        Ok(path) => {
            println!("Found file at {}!", path);
            path
        }
        Err(MyError::UserInputError) => {
            eprintln!("Error: Incorrect user input!");
            return;
        }
        Err(MyError::FileNotFoundError) => {
            eprintln!("Error: File not found!");
            return;
        }
        Err(MyError::NotAJsonFileError) => {
            eprintln!("Error: Your file is not a JSON file!");
            return;
        }
        _ => {
            eprintln!("Unexpected error occurred while trying to find json file!");
            return;
        }
    };

    println!("");
    if let Err(err) = put_recipes_from_path_into_md_file(&path, directory, file_name_md) {
        match err {
            MyError::FileCopyError(path) => eprintln!("Error while copying file {}!", path),
            MyError::FileOpenError(file_name) => eprintln!("Could not open file {}!", file_name),
            MyError::JsonParsingError(path) => eprintln!("Error while parsing JSON file {}!", path),
            _ => eprint!("Unexpected error occurred while copying the file!"),
        }
        return;
    }
    println!("Successfully added recipes to md file!");
    
    println!("");
    if let Err(err) = put_recipes_from_path_into_adoc_file(&path, directory, file_name_adoc) {
        match err {
            MyError::FileCopyError(path) => eprintln!("Error while copying file {}!", path),
            MyError::FileOpenError(file_name) => eprintln!("Could not open file {}!", file_name),
            MyError::JsonParsingError(path) => eprintln!("Error while parsing JSON file {}!", path),
            _ => eprint!("Unexpected error occurred while copying the file!"),
        }
        return;
    }
    println!("Successfully added recipes to adoc file!");
    println!("");
}
