use codex_tools::JsonSchema;
use codex_tools::ResponsesApiTool;
use codex_tools::ToolSpec;
use serde_json::json;
use std::collections::BTreeMap;

pub const DOPAX_EXPERIENCE_MANAGER_TOOL_NAME: &str = "dopax_experience_manager";

pub fn create_dopax_experience_manager_tool() -> ToolSpec {
    let properties = BTreeMap::from([
        (
            "action".to_string(),
            JsonSchema::string_enum(
                vec![
                    json!("create"),
                    json!("update"),
                    json!("delete"),
                    json!("list"),
                ],
                Some(
                    "Action to perform on user experience events: create, update, delete, or list."
                        .to_string(),
                ),
            ),
        ),
        (
            "id".to_string(),
            JsonSchema::string(Some(
                "Unique event identifier (required for update/delete, e.g. exp_20260802_01)."
                    .to_string(),
            )),
        ),
        (
            "title".to_string(),
            JsonSchema::string(Some(
                "Short descriptive title of the experience or event.".to_string(),
            )),
        ),
        (
            "status".to_string(),
            JsonSchema::string_enum(
                vec![json!("ongoing"), json!("completed"), json!("expired")],
                Some(
                    "Current status of the experience: ongoing, completed, or expired.".to_string(),
                ),
            ),
        ),
        (
            "start_date".to_string(),
            JsonSchema::string(Some("Start date in YYYY-MM-DD format.".to_string())),
        ),
        (
            "end_date".to_string(),
            JsonSchema::string(Some(
                "End date in YYYY-MM-DD format (used for auto-expiration).".to_string(),
            )),
        ),
        (
            "summary".to_string(),
            JsonSchema::string(Some(
                "Detailed summary of progress, milestone, or event context.".to_string(),
            )),
        ),
    ]);

    ToolSpec::Function(ResponsesApiTool {
        name: DOPAX_EXPERIENCE_MANAGER_TOOL_NAME.to_string(),
        description: r#"Manages user experiences, project timelines, and major milestone events.
Allows creating, updating, deleting, or listing active user experiences stored in the Dopax Home sandbox.
Dopax automatically injects current time and active ongoing experiences into the developer context."#
            .to_string(),
        strict: false,
        defer_loading: None,
        parameters: JsonSchema::object(
            properties,
            Some(vec!["action".to_string()]),
            Some(false.into()),
        ),
        output_schema: None,
    })
}
