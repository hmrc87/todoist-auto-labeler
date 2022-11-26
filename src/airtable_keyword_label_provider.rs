pub mod airtable_keyword_label_provider {
    use crate::KeywordLabelCombo;
    use serde_derive::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct AirtableResponse {
        records: Records,
        #[serde(default)]
        offset: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct Records {
        array: Vec<AirtableEntries>,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct AirtableEntries {
        fields: AirtableKeywordLabel,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct AirtableKeywordLabel {
        #[serde(default)]
        #[serde(rename = "Keyword")]
        keyword: String,
        #[serde(default)]
        #[serde(rename = "Label")]
        label: String,
    }

    impl From<AirtableKeywordLabel> for KeywordLabelCombo {
        fn from(a: AirtableKeywordLabel) -> Self {
            Self {
                keyword: a.keyword,
                label: a.label,
            }
        }
    }

    pub async fn get_keyword_label_combos(
        airtable_url: &str,
        airtable_token: &str,
    ) -> Vec<KeywordLabelCombo> {
        let client: reqwest::Client = reqwest::Client::new();
       
        let mut final_results = Vec::new();

        let response = client
            .get(airtable_url)
            .header("Authorization", "Bearer ".to_owned() + &airtable_token)
            .send()
            .await;      
        
        match response {
            Ok(res) => {
                let response = res.json::<AirtableResponse>().await.unwrap();
                final_results.extend(
                    response
                        .records
                        .array
                        .iter()
                        .map(|a| KeywordLabelCombo {
                            keyword: String::from(&a.fields.keyword),
                            label: String::from(&a.fields.label),
                        })
                        .collect::<Vec::<KeywordLabelCombo>>(),
                );

                let mut offset = response.offset;               
                while !offset.is_empty() {
                    
                    let url = airtable_url.to_owned() + "&offset=" + &offset;
                    let res = client
                        .get(url)
                        .header("Authorization", "Bearer ".to_owned() + &airtable_token)
                        .send()
                        .await;

                    match res {
                        Ok(res) => {
                            let response = res.json::<AirtableResponse>().await.unwrap();
                            final_results.extend(
                                response
                                    .records
                                    .array
                                    .iter()
                                    .map(|a| KeywordLabelCombo {
                                        keyword: String::from(&a.fields.keyword),
                                        label: String::from(&a.fields.label),
                                    })
                                    .collect::<Vec::<KeywordLabelCombo>>(),
                            );
                            offset = response.offset
                        }
                        Err(e) => {
                            println!("Error retrieving Airtable data: {:?}", e);
                            return Vec::<KeywordLabelCombo>::with_capacity(1);
                        }
                    }
                }

                final_results
            }
            Err(e) => {
                println!("Error retrieving Airtable data: {:?}", e);
                return Vec::<KeywordLabelCombo>::with_capacity(1);
            }
        }
    }
}
