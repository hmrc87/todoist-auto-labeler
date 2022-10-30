extern crate hyper;
extern crate hyper_rustls;
extern crate dotenv;

mod todoist_api_adapter;
use crate::todoist_api_adapter::todoist_api_adapter::*;

mod airtable_keyword_label_provider;
use crate::airtable_keyword_label_provider::airtable_keyword_label_provider::*;

#[derive(Debug, Clone)]
pub struct KeywordLabelCombo{
    Keyword: String,
    Label: String
}

 
#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to read .env file");
    
    let todoist_token = std::env::var("todoist_bearer_token")
        .expect("todoist_bearer_token environment variable must be set.");
    let todoist_project_id = std::env::var("todoist_project_id")
        .expect("todoist_project_id environment variable must be set.");
    let airtable_token: String = std::env::var("airtable_api_key")
        .expect("airtable_api_key environment variable must be set.");
    let airtable_base_url: String = std::env::var("airtable_base_url")
        .expect("airtable_base_url environment variable must be set.");    

    let keyword_label_combos = get_keyword_label_combos(
        &airtable_base_url, 
        &airtable_token)
        .await;
    
    println!("Keyword-Label combo count: {}", keyword_label_combos.len());
    update_todoist_labels(keyword_label_combos, &todoist_project_id, &todoist_token).await;
}

async fn update_todoist_labels(
    keyword_label_combos: Vec<KeywordLabelCombo>,
    todoist_project_id: &str, 
    todoist_token: &str) 
    -> bool
    {

    let original_todoist_tasks = get_todoist_tasks(todoist_project_id, todoist_token).await;
    let mut updated_todoist_tasks: Vec<UpdateTodoistTask> = Vec::new();

    for mut task in original_todoist_tasks {
        let matched_keyword = does_match(&task.content, &keyword_label_combos);
        
        match matched_keyword {
            Some (combo) =>{
                let mut push_label = true;
                for label in &task.labels{
                    if label.contains(&combo.Label) {
                        push_label = false;
                        continue;
                    };
                }

                if push_label {
                    task.labels.push(String::from(&combo.Label));
                    updated_todoist_tasks.push(UpdateTodoistTask::from(task));
                }
            }
            None => {}
        }
    };

    println!("Updating : {} tasks.", updated_todoist_tasks.len());
    for updated_task in updated_todoist_tasks{
        update_todoist_task(&updated_task, todoist_token).await;
    }
    true 
}

fn does_match<'a>(content: &str, keyword_label_combos: &'a[KeywordLabelCombo]) -> Option<&'a KeywordLabelCombo> {  
    for keyword_label_combo in keyword_label_combos {
        if content.to_lowercase().contains(&keyword_label_combo.Keyword.to_lowercase()) {
            return Some(keyword_label_combo);
        }
    }
    return None;
}