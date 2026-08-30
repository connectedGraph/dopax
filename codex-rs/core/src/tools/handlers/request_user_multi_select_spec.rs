use codex_protocol::config_types::ModeKind;
use codex_protocol::request_user_input::RequestUserInputArgs;
use codex_tools::JsonSchema;
use codex_tools::ResponsesApiTool;
use codex_tools::ToolSpec;
use std::collections::BTreeMap;

pub const REQUEST_USER_MULTI_SELECT_TOOL_NAME: &str = "request_user_multi_select";
pub const MIN_AUTO_RESOLUTION_MS: u64 = 60_000;
pub const MAX_AUTO_RESOLUTION_MS: u64 = 240_000;

pub fn create_request_user_multi_select_tool(description: String) -> ToolSpec {
    let option_props = BTreeMap::from([
        (
            "label".to_string(),
            JsonSchema::string(Some("User-facing label (1-5 words).".to_string())),
        ),
        (
            "description".to_string(),
            JsonSchema::string(Some(
                "One short sentence explaining impact/tradeoff if selected.".to_string(),
            )),
        ),
    ]);

    let options_schema = JsonSchema::array(JsonSchema::object(
            option_props,
            Some(vec!["label".to_string(), "description".to_string()]),
            Some(false.into()),
        ), Some(
            "Provide 2-5 mutually exclusive or concurrent choices. The user can toggle multiple checkboxes."
                .to_string(),
        ));

    let question_props = BTreeMap::from([
        (
            "id".to_string(),
            JsonSchema::string(Some(
                "Stable identifier for mapping answers (snake_case).".to_string(),
            )),
        ),
        (
            "header".to_string(),
            JsonSchema::string(Some(
                "Short header label shown in the UI (12 or fewer chars).".to_string(),
            )),
        ),
        (
            "question".to_string(),
            JsonSchema::string(Some(
                "Single-sentence prompt shown to the user.".to_string(),
            )),
        ),
        ("options".to_string(), options_schema),
    ]);

    let questions_schema = JsonSchema::array(
        JsonSchema::object(
            question_props,
            Some(vec![
                "id".to_string(),
                "header".to_string(),
                "question".to_string(),
                "options".to_string(),
            ]),
            Some(false.into()),
        ),
        Some("Questions to show the user. Prefer 1 and do not exceed 3".to_string()),
    );

    let auto_resolution_ms_schema = JsonSchema::number(Some(format!(
        "Optional auto-resolution window in milliseconds, from {MIN_AUTO_RESOLUTION_MS} to {MAX_AUTO_RESOLUTION_MS}. Include this only when the question is useful but non-blocking."
    )));

    let properties = BTreeMap::from([
        ("questions".to_string(), questions_schema),
        ("autoResolutionMs".to_string(), auto_resolution_ms_schema),
    ]);

    ToolSpec::Function(ResponsesApiTool {
        name: REQUEST_USER_MULTI_SELECT_TOOL_NAME.to_string(),
        description,
        strict: false,
        defer_loading: None,
        parameters: JsonSchema::object(
            properties,
            Some(vec!["questions".to_string()]),
            Some(false.into()),
        ),
        output_schema: None,
    })
}

pub fn normalize_request_user_multi_select_args(
    mut args: RequestUserInputArgs,
) -> Result<RequestUserInputArgs, String> {
    let missing_options = args
        .questions
        .iter()
        .any(|question| question.options.as_ref().is_none_or(Vec::is_empty));
    if missing_options {
        return Err(
            "request_user_multi_select requires non-empty options for every question".to_string(),
        );
    }

    for question in &mut args.questions {
        question.is_other = true;
        question.is_multi_select = true; // Crucial: enforce multi-select behavior
    }

    if let Some(auto_resolution_ms) = args.auto_resolution_ms {
        let clamped_auto_resolution_ms =
            auto_resolution_ms.clamp(MIN_AUTO_RESOLUTION_MS, MAX_AUTO_RESOLUTION_MS);
        if clamped_auto_resolution_ms != auto_resolution_ms {
            args.auto_resolution_ms = Some(clamped_auto_resolution_ms);
        }
    }

    Ok(args)
}

pub fn request_user_multi_select_tool_description(available_modes: &[ModeKind]) -> String {
    let allowed_modes = format_allowed_modes(available_modes);
    format!(
        "Request user input for one to three short multi-select questions and wait for the response. Displays a list of options with checkboxes where the user can toggle multiple selections before submitting. This tool is only available in {allowed_modes}."
    )
}

fn format_allowed_modes(available_modes: &[ModeKind]) -> String {
    let mode_names: Vec<&str> = available_modes
        .iter()
        .map(|mode| mode.display_name())
        .collect();

    match mode_names.as_slice() {
        [] => "no modes".to_string(),
        [mode] => format!("{mode} mode"),
        [first, second] => format!("{first} or {second} mode"),
        [..] => format!("modes: {}", mode_names.join(",")),
    }
}
