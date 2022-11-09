pub mod todoist_api_adapter {
    use serde_derive::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct TodoistTask {
        pub project_id: String,
        pub content: String,
        pub labels: Vec<String>,
        pub id: String,
    }
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct UpdateTodoistTask {
        id: String,
        labels: Vec<String>,
    }

    impl From<TodoistTask> for UpdateTodoistTask {
        fn from(t: TodoistTask) -> Self {
            Self {
                id: t.id,
                labels: t.labels,
            }
        }
    }

    pub async fn get_todoist_tasks(project_id: &str, token: &str) -> Vec<TodoistTask> {
        let client = reqwest::Client::new();

        let response = client
            .get(&(format!("https://api.todoist.com/rest/v2/tasks")))
            .header("Authorization", "Bearer ".to_owned() + &token)
            .send()
            .await;

        let all_tasks = response.unwrap().json::<Vec<TodoistTask>>().await.unwrap();
        let filtered_tasks = all_tasks
            .into_iter()
            .filter(|x| x.project_id.eq(&project_id.to_string()))
            .collect::<Vec<TodoistTask>>();
        filtered_tasks
    }

    pub async fn update_todoist_task(task: &UpdateTodoistTask, token: &str) -> bool {
        let client = reqwest::Client::new();
        let url = "https://api.todoist.com/rest/v2/tasks/".to_string() + &task.id;

        let res = client
            .post(&url)
            .header("Authorization", "Bearer ".to_owned() + &token)
            .json(&task)
            .send()
            .await;

        match res {
            Ok(_res) => true,
            Err(e) => {
                println!("Error: {:?}", e);
                false
            }
        }
    }
}
