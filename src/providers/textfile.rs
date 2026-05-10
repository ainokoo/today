use std::path::{Path, PathBuf};
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufRead, BufWriter, Write};


use crate::EventProvider;
use crate::events::{Event, Category, EventKind};
use crate::filters::EventFilter;
use crate::providers::EventProviderError;

enum ReadingState {
    Date,
    Description,
    Category,
    Separator,
}

pub struct TextFileProvider {
    name: String,
    path: PathBuf,
}

impl TextFileProvider {
    pub fn new(name: &str, path: &Path) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_path_buf()
        }
    }
}

impl EventProvider for TextFileProvider {
    fn name(&self) -> String {
        self.name.clone()
    }
    
    fn get_events(&self, filter: &EventFilter, events: &mut Vec<Event>) {
        let f = File::open(self.path.clone()).expect("path to text file");
        let reader = BufReader::new(f);
        let mut state = ReadingState::Date;
        let mut date_string = String::new();
        let mut description = String::new();
        let mut category_string = String::new();

        for line_result in reader.lines() {
            let line = line_result.expect("read line");
            match state {
                ReadingState::Date => {
                    date_string = line;
                    state = ReadingState::Description;
                },
                ReadingState::Description => {
                    description = line;
                    state = ReadingState::Category;
                },
                ReadingState::Category => {
                    category_string = line;
                    state = ReadingState::Separator;
                },
                ReadingState::Separator => {
                    match chrono::NaiveDate::parse_from_str(&date_string, "%F") {
                        Ok(date) => {
                            let category = Category::from_str(&category_string);
                            let event = Event::new_singular(
                                date,
                                description.clone(),
                                category);
                                
                                if filter.accepts(&event) {
                                    events.push(event);
                                }
                        },
                        Err(_) => {
                            eprintln!("Invalid timestamp '{}'", date_string);
                        }
                    }
                    state = ReadingState::Date;
                },
            }
        }
    }

    fn is_add_supported(&self) -> bool { true }

    fn add_event(&self, event: &Event) -> Result<(), EventProviderError> {
        if !self.is_add_supported() {
            return Err(EventProviderError::OperationNotSupported);
        }
        let file = OpenOptions::new()
            .append(true)
            .open(self.path.clone())
            .expect("path to text file for writing");
        let mut writer = BufWriter::new(file);

        return match event.kind {
            EventKind::Singular(date) => {
            writeln!(writer, "{}", date.to_string());
            writeln!(writer, "{}", event.description());
            writeln!(writer, "{}", event.category());
            writeln!(writer, "");
            Ok(())
        },
        _ => Err(EventProviderError::OperationNotSupported)
        };
        
    }


}