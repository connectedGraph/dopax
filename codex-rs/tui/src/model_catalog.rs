use codex_config::CustomModelPreset;
use codex_protocol::openai_models::ModelPreset;
use codex_protocol::openai_models::ReasoningEffort;
use codex_protocol::openai_models::ReasoningEffortPreset;
use codex_protocol::openai_models::SPEED_TIER_FAST;
use std::convert::Infallible;

#[derive(Debug, Clone)]
pub(crate) struct ModelCatalog {
    models: Vec<ModelPreset>,
}

impl ModelCatalog {
    pub(crate) fn new(models: Vec<ModelPreset>) -> Self {
        Self { models }
    }

    /// Merges custom model presets from config.toml into the catalog.
    ///
    /// Custom presets override the metadata of a matching model from the
    /// dynamic catalog, and are appended when no matching model exists yet.
    pub(crate) fn merge_custom_presets(&mut self, custom: Vec<CustomModelPreset>) {
        if custom.is_empty() {
            return;
        }

        for preset in custom {
            let entry = self
                .models
                .iter_mut()
                .find(|model| model.model == preset.model);

            match entry {
                Some(model) => apply_custom_preset(model, &preset),
                None => {
                    if let Some(model) = custom_preset_to_model(&preset) {
                        self.models.push(model);
                    }
                }
            }
        }
    }

    pub(crate) fn try_list_models(&self) -> Result<Vec<ModelPreset>, Infallible> {
        Ok(self.models.clone())
    }
}

fn apply_custom_preset(model: &mut ModelPreset, preset: &CustomModelPreset) {
    if let Some(description) = &preset.description {
        model.description = description.clone();
    }
    if let Some(reasoning_efforts) = &preset.reasoning_efforts {
        model.supported_reasoning_efforts = reasoning_efforts
            .iter()
            .map(|effort| ReasoningEffortPreset {
                effort: effort.clone(),
                description: effort.to_string(),
            })
            .collect();
    }
    if let Some(default_reasoning_effort) = &preset.default_reasoning_effort {
        model.default_reasoning_effort = default_reasoning_effort.clone();
    }
    if let Some(supports_fast_mode) = preset.supports_fast_mode {
        model.additional_speed_tiers = if supports_fast_mode {
            if !model
                .additional_speed_tiers
                .iter()
                .any(|tier| tier == SPEED_TIER_FAST)
            {
                vec![SPEED_TIER_FAST.to_string()]
            } else {
                model.additional_speed_tiers.clone()
            }
        } else {
            model
                .additional_speed_tiers
                .clone()
                .into_iter()
                .filter(|tier| tier != SPEED_TIER_FAST)
                .collect()
        };
    }
    if let Some(show_in_picker) = preset.show_in_picker {
        model.show_in_picker = show_in_picker;
    }
}

fn custom_preset_to_model(preset: &CustomModelPreset) -> Option<ModelPreset> {
    let model = preset.model.clone();
    if model.is_empty() {
        return None;
    }

    let supported_reasoning_efforts = preset
        .reasoning_efforts
        .as_ref()
        .map(|efforts| {
            efforts
                .iter()
                .map(|effort| ReasoningEffortPreset {
                    effort: effort.clone(),
                    description: effort.to_string(),
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|| {
            vec![ReasoningEffortPreset {
                effort: ReasoningEffort::default(),
                description: ReasoningEffort::default().to_string(),
            }]
        });

    let supports_fast_mode = preset.supports_fast_mode.unwrap_or(false);
    let additional_speed_tiers = if supports_fast_mode {
        vec![SPEED_TIER_FAST.to_string()]
    } else {
        Vec::new()
    };

    Some(ModelPreset {
        id: model.clone(),
        model: model.clone(),
        display_name: model.clone(),
        description: preset.description.clone().unwrap_or_default(),
        model_specialty: None,
        default_reasoning_effort: preset.default_reasoning_effort.clone().unwrap_or_default(),
        supported_reasoning_efforts,
        supports_personality: false,
        additional_speed_tiers,
        service_tiers: Vec::new(),
        default_service_tier: None,
        is_default: false,
        upgrade: None,
        show_in_picker: preset.show_in_picker.unwrap_or(true),
        multi_agent_version: None,
        availability_nux: None,
        supported_in_api: true,
        input_modalities: Vec::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_preset(model: &str) -> ModelPreset {
        ModelPreset {
            id: model.to_string(),
            model: model.to_string(),
            display_name: model.to_string(),
            description: "base".to_string(),
            model_specialty: None,
            default_reasoning_effort: ReasoningEffort::Medium,
            supported_reasoning_efforts: vec![ReasoningEffortPreset {
                effort: ReasoningEffort::Medium,
                description: "medium".to_string(),
            }],
            supports_personality: false,
            additional_speed_tiers: Vec::new(),
            service_tiers: Vec::new(),
            default_service_tier: None,
            is_default: false,
            upgrade: None,
            show_in_picker: true,
            multi_agent_version: None,
            availability_nux: None,
            supported_in_api: true,
            input_modalities: Vec::new(),
        }
    }

    #[test]
    fn merge_custom_presets_overrides_matching_model() {
        let mut catalog = ModelCatalog::new(vec![base_preset("gpt-x")]);
        catalog.merge_custom_presets(vec![CustomModelPreset {
            model: "gpt-x".to_string(),
            description: Some("custom description".to_string()),
            context_window: Some(200_000),
            reasoning_efforts: None,
            default_reasoning_effort: None,
            supports_fast_mode: Some(true),
            show_in_picker: None,
        }]);

        let models = catalog
            .try_list_models()
            .expect("catalog should list models");
        assert_eq!(models.len(), 1);
        assert_eq!(models[0].description, "custom description");
        assert!(models[0].supports_fast_mode());
    }

    #[test]
    fn merge_custom_presets_appends_unknown_model() {
        let mut catalog = ModelCatalog::new(vec![base_preset("gpt-x")]);
        catalog.merge_custom_presets(vec![CustomModelPreset {
            model: "gpt-unknown".to_string(),
            description: Some("relay model".to_string()),
            context_window: None,
            reasoning_efforts: Some(vec![ReasoningEffort::High]),
            default_reasoning_effort: Some(ReasoningEffort::High),
            supports_fast_mode: Some(false),
            show_in_picker: Some(true),
        }]);

        let models = catalog
            .try_list_models()
            .expect("catalog should list models");
        assert_eq!(models.len(), 2);
        let custom = models
            .iter()
            .find(|model| model.model == "gpt-unknown")
            .expect("custom model should be appended");
        assert_eq!(custom.description, "relay model");
        assert_eq!(custom.default_reasoning_effort, ReasoningEffort::High);
        assert_eq!(custom.supported_reasoning_efforts.len(), 1);
        assert!(!custom.supports_fast_mode());
    }

    #[test]
    fn merge_custom_presets_noop_on_empty() {
        let mut catalog = ModelCatalog::new(vec![base_preset("gpt-x")]);
        catalog.merge_custom_presets(Vec::new());
        let models = catalog
            .try_list_models()
            .expect("catalog should list models");
        assert_eq!(models.len(), 1);
    }
}
