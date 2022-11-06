pub mod airtable_keyword_label_provider {
    use serde_derive::{Serialize, Deserialize};

    use crate::{KeywordLabelCombo};
   
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AirtableResponse {
        records: Records
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct Records {
        array: Vec<AirtableEntries>
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct AirtableEntries {
        fields: AirtableKeywordLabel
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct AirtableKeywordLabel{
        #[serde(default)]
        #[serde(rename = "Keyword")]
        keyword: String,
        #[serde(default)]
        #[serde(rename = "Label")]
        label: String
    }

    impl From<AirtableKeywordLabel> for KeywordLabelCombo{
        fn from(a: AirtableKeywordLabel) -> Self {
            Self {
              keyword: a.keyword,
              label: a.label
            }
          }
    }

    pub async fn get_keyword_label_combos(
        airtable_url: &str,
        airtable_token: &str,
    ) -> Vec<KeywordLabelCombo> {
        let client: reqwest::Client = reqwest::Client::new();
        
        // TODO: 100 values max -> if more use pagination
        let response = client
            .get(airtable_url)
            .header("Authorization", "Bearer ".to_owned() + &airtable_token)
            .send()
            .await;

        match response {
            Ok(res) => {
                let airtable_response = res.json::<AirtableResponse>().await.unwrap();
                let result : Vec<KeywordLabelCombo> = airtable_response.records.array
                    .iter()
                    .map(|a| KeywordLabelCombo{ keyword: String::from(&a.fields.keyword), label: String::from(&a.fields.label)})
                    .collect();
                result

            }
            Err(e) =>{
                println!("Error retrieving Airtable data: {:?}",e);
                return Vec::<KeywordLabelCombo>::with_capacity(1);
            }
        }
    }
}
