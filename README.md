# Rust Toodoist-Auto-Labeler
I am using Amazon Alexa to add items to my shopping list. These items are synced with Todoist. 
The integration works fine, however it is misssing something my partner and I really need: Labels. 
Labels help us organize the shopping process so that's why we are adding them manually. In order to automate this tedious task - and to learn Rust - I decided
to implement this tool.

## User stories
- I want to be able to provide a keyword-label list to google sheets that will categorize the items on my list. 
- The keyword-label mapping should be case-insensitive and also add the correct label if am using a (simple) plural or singular form of an item (apple - apples, banana - bananas)
