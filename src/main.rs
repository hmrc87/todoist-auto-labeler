extern crate dotenv;
extern crate hyper;
extern crate hyper_rustls;

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
    // Read from .env file if available
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

    println!("keyword-label combo count: {}", keyword_label_combos.len());

    let todoist_tasks = get_todoist_tasks(&todoist_project_id, &todoist_token).await;
    let todoist_tasks_without_alexa_label = filter_label("Alexa", todoist_tasks);

    let updated_todoist_tasks = update_todoist_labels(todoist_tasks_without_alexa_label, keyword_label_combos);

    for updated_task in updated_todoist_tasks {
        update_todoist_task(&updated_task, &todoist_token).await;
    }
}

fn filter_label(label: &str, tasks: Vec<TodoistTask>) -> Vec<TodoistTask> {
    let mut updated_tasks = Vec::new();
    for task in tasks {
        let mut updated_task = task.clone();
        updated_task.labels.retain(|value| !value.contains(label));
        updated_tasks.push(updated_task);
    }
    updated_tasks
}

fn update_todoist_labels(
    original_todoist_tasks: Vec<TodoistTask>,
    keyword_label_combos: Vec<KeywordLabelCombo>,
) -> Vec<UpdateTodoistTask> {
    let mut updated_todoist_tasks: Vec<UpdateTodoistTask> = Vec::new();

    for mut task in original_todoist_tasks {
        let matched_keyword = get_match(&task.content, &keyword_label_combos);

        match matched_keyword {
            Some(combo) => {
                let mut do_push_label = true;
                for label in &task.labels {
                    if label.contains(&combo.label) {
                        do_push_label = false;
                        continue;
                    };
                }

                if do_push_label {
                    task.labels.push(String::from(&combo.label));
                    updated_todoist_tasks.push(UpdateTodoistTask::from(task));
                }
            }
            None => {}
        }
    }
    println!("Updating : {} tasks.", updated_todoist_tasks.len());
    updated_todoist_tasks
}

fn get_match<'a>(
    search_term: &str,
    keyword_label_combos: &'a [KeywordLabelCombo],
) -> Option<&'a KeywordLabelCombo> {
    for keyword_label_combo in keyword_label_combos {
        if search_term
            .to_lowercase()
            .contains(&keyword_label_combo.keyword.to_lowercase())
        {
            return Some(keyword_label_combo);
        }
    }
    return None;
}
