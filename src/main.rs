extern crate dotenv;
extern crate hyper;
extern crate hyper_rustls;
extern crate regex;
use regex::Regex;

mod todoist_api_adapter;
use crate::todoist_api_adapter::todoist_api_adapter::*;

mod airtable_keyword_label_provider;
use crate::airtable_keyword_label_provider::airtable_keyword_label_provider::*;

#[derive(Debug, Clone)]
pub struct KeywordLabelCombo {
    keyword: String,
    label: String,
}

#[tokio::main]
async fn main() {
    println!(r"
        _______        _       _     _                      _        _       _          _           
       |__   __|      | |     (_)   | |          /\        | |      | |     | |        | |          
          | | ___   __| | ___  _ ___| |_        /  \  _   _| |_ ___ | | __ _| |__   ___| | ___ _ __ 
          | |/ _ \ / _` |/ _ \| / __| __|      / /\ \| | | | __/ _ \| |/ _` | '_ \ / _ \ |/ _ \ '__|
          | | (_) | (_| | (_) | \__ \ |_      / ____ \ |_| | || (_) | | (_| | |_) |  __/ |  __/ |   
          |_|\___/ \__,_|\___/|_|___/\__|    /_/    \_\__,_|\__\___/|_|\__,_|_.__/ \___|_|\___|_| 
          version 1.0 (A simple tool to automatically add labels to Todoist tasks and learn Rust :D)                                                                                                                                                                                           
      ");
    dotenv::dotenv().ok();

    let todoist_token = std::env::var("todoist_bearer_token")
        .expect("todoist_bearer_token environment variable must be set.");
    let todoist_project_id = std::env::var("todoist_project_id")
        .expect("todoist_project_id environment variable must be set.");
    let airtable_token: String = std::env::var("airtable_api_key")
        .expect("airtable_api_key environment variable must be set.");
    let airtable_base_url: String = std::env::var("airtable_base_url")
        .expect("airtable_base_url environment variable must be set.");

    let keyword_label_combos = get_keyword_label_combos(&airtable_base_url, &airtable_token).await;

    println!("Airtable Keyword-label-combo count: {}", keyword_label_combos.len());

    let todoist_tasks = get_todoist_tasks(&todoist_project_id, &todoist_token).await;
    println!("Active Todoist-Task count: {}", todoist_tasks.len());
    let todoist_tasks_without_alexa_label = filter_label("Alexa", todoist_tasks);

    let updated_todoist_tasks = update_labels(todoist_tasks_without_alexa_label, keyword_label_combos);

    for updated_task in updated_todoist_tasks {
        update_todoist_task(&updated_task, &todoist_token).await;
    }
}

fn filter_label(label: &str, tasks: Vec<TodoistTask>) -> Vec<TodoistTask> {
    println!("removing label {} from all tasks", label);
    let mut updated_tasks = Vec::new();
    for task in tasks {
        let mut updated_task = task.clone();
        updated_task.labels.retain(|value| !value.contains(label));
        updated_tasks.push(updated_task);
    }
    updated_tasks
}

fn update_labels(
    tasks: Vec<TodoistTask>,
    keyword_label_combos: Vec<KeywordLabelCombo>,
) -> Vec<UpdateTodoistTask> {
    let mut updated_tasks: Vec<UpdateTodoistTask> = Vec::new();

    for mut task in tasks {
        let matched_keyword = get_match(&task.content, &keyword_label_combos);

        match matched_keyword {
            Some(combo) => {
                println!("Matched keyword: {:?} with task {:?}", matched_keyword.unwrap().keyword, task.content);
                let mut push_new_label = true;
                
                for label in &task.labels {
                    if label.contains(&combo.label) {
                        push_new_label = false;
                        continue;
                    };
                }

                if push_new_label {
                    task.labels.push(String::from(&combo.label));
                }

                updated_tasks.push(UpdateTodoistTask::from(task));
            }
            None => {}
        }
    }
    println!("Updating {} task(s) ...", updated_tasks.len());
    updated_tasks
}

fn get_match<'a>(
    search_term: &str,
    keyword_label_combos: &'a Vec<KeywordLabelCombo>,
) -> Option<&'a KeywordLabelCombo> {
    for keyword_label_combo in keyword_label_combos.iter() {
        if keyword_label_combo.keyword == "" {
            continue;
        }
        let regex_string = "(?i)".to_owned() + &keyword_label_combo.keyword;
        let regex = Regex::new(&regex_string);
        match regex {
            Ok(res) => match res.find(search_term) {
                Some(_) => return Some(keyword_label_combo),
                None =>  continue
            },
            Err(_) => return None,
        }
        
    }
    return None;
}

#[cfg(test)]
mod tests {
    use crate::{KeywordLabelCombo, get_match};

    #[test]
    fn get_match_matches() {
        
        // Arrange
        let search_term = "Bananas";
        let mut keyword_label_combos = Vec::new();

        let banana_combo = KeywordLabelCombo{
            keyword : String::from("banana"),
            label : String::from("Fruits"),
        };

        let tomato_combo = KeywordLabelCombo{
            keyword : String::from("Tomato"),
            label : String::from("Fruits(seriously!)"),
        };

        let banana_clone = banana_combo.clone();

        keyword_label_combos.push(tomato_combo);
        keyword_label_combos.push(banana_combo);

        // Act
        let result = get_match(search_term, &keyword_label_combos)
                    .unwrap();

        // Assert
        assert_eq!(&result.keyword, &banana_clone.keyword);
    }
}

