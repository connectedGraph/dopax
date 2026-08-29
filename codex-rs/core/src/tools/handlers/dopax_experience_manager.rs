use crate::experiences::ExperienceItem;
use crate::experiences::ExperienceStore;
use crate::function_tool::FunctionCallError;
use crate::tools::context::FunctionToolOutput;
use crate::tools::context::ToolInvocation;
use crate::tools::context::ToolPayload;
use crate::tools::context::boxed_tool_output;
use crate::tools::handlers::dopax_experience_manager_spec::DOPAX_EXPERIENCE_MANAGER_TOOL_NAME;
use crate::tools::handlers::dopax_experience_manager_spec::create_dopax_experience_manager_tool;
use crate::tools::handlers::parse_arguments;
use crate::tools::registry::CoreToolRuntime;
use crate::tools::registry::ToolExecutor;
use codex_tools::ToolName;
use codex_tools::ToolSpec;
use serde::Deserialize;

pub struct DopaxExperienceManagerHandler;

#[derive(Debug, Deserialize)]
struct DopaxExperienceArgs {
    action: String,
    id: Option<String>,
    title: Option<String>,
    status: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
    summary: Option<String>,
    tags: Option<Vec<String>>,
}

impl ToolExecutor<ToolInvocation> for DopaxExperienceManagerHandler {
    fn tool_name(&self) -> ToolName {
        ToolName::plain(DOPAX_EXPERIENCE_MANAGER_TOOL_NAME)
    }

    fn spec(&self) -> ToolSpec {
        create_dopax_experience_manager_tool()
    }

    fn handle(&self, invocation: ToolInvocation) -> codex_tools::ToolExecutorFuture<'_> {
        Box::pin(async move {
            let ToolInvocation { payload, .. } = invocation;
            let ToolPayload::Function { arguments } = payload else {
                return Err(FunctionCallError::RespondToModel(format!(
                    "{DOPAX_EXPERIENCE_MANAGER_TOOL_NAME} received unsupported payload"
                )));
            };

            let args: DopaxExperienceArgs = parse_arguments(&arguments)?;

            match args.action.as_str() {
                "list" => {
                    let items = ExperienceStore::load_and_purge();
                    let json = serde_json::to_string_pretty(&items).unwrap_or_default();
                    Ok(boxed_tool_output(FunctionToolOutput::from_text(
                        json,
                        Some(true),
                    )))
                }
                "create" | "update" => {
                    let id = args.id.unwrap_or_else(|| {
                        let today = chrono::Utc::now().format("%Y%m%d").to_string();
                        format!("exp_{today}_{}", uuid::Uuid::new_v4().simple())
                    });
                    let item = ExperienceItem {
                        id: id.clone(),
                        title: args.title.unwrap_or_else(|| "Untitled Experience".to_string()),
                        status: args.status.unwrap_or_else(|| "ongoing".to_string()),
                        start_date: args.start_date.unwrap_or_else(|| chrono::Utc::now().format("%Y-%m-%d").to_string()),
                        end_date: args.end_date.unwrap_or_else(|| chrono::Utc::now().format("%Y-%m-%d").to_string()),
                        summary: args.summary.unwrap_or_default(),
                        tags: args.tags.unwrap_or_default(),
                    };

                    match ExperienceStore::add(item.clone()) {
                        Ok(saved) => {
                            let json = serde_json::to_string_pretty(&saved).unwrap_or_default();
                            Ok(boxed_tool_output(FunctionToolOutput::from_text(
                                format!("Successfully saved experience:\n{json}"),
                                Some(true),
                            )))
                        }
                        Err(err) => Err(FunctionCallError::RespondToModel(format!(
                            "Failed to save experience: {err}"
                        ))),
                    }
                }
                "delete" => {
                    let Some(id) = args.id.as_deref() else {
                        return Err(FunctionCallError::RespondToModel(
                            "delete action requires an `id`".to_string(),
                        ));
                    };
                    match ExperienceStore::delete(id) {
                        Ok(true) => Ok(boxed_tool_output(FunctionToolOutput::from_text(
                            format!("Successfully deleted experience with id `{id}`."),
                            Some(true),
                        ))),
                        Ok(false) => Ok(boxed_tool_output(FunctionToolOutput::from_text(
                            format!("No experience found with id `{id}`."),
                            Some(false),
                        ))),
                        Err(err) => Err(FunctionCallError::RespondToModel(format!(
                            "Failed to delete experience: {err}"
                        ))),
                    }
                }
                other => Err(FunctionCallError::RespondToModel(format!(
                    "Unknown action `{other}`. Supported actions: create, update, delete, list"
                ))),
            }
        })
    }
}

impl CoreToolRuntime for DopaxExperienceManagerHandler {}
