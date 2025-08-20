use crate::journal::event;
use crate::state;

impl Into<state::Engineering> for event::LoadoutModuleEngineering {
    fn into(self) -> state::Engineering {
        state::Engineering {
            engineer: self.engineer.unwrap_or_default(),
            blueprint_name: self.blueprint_name.split('_').skip(1).next().unwrap_or_default().to_string(),
            level: self.level,
            quality: self.quality,
            experimental_effect: self.experimental_effect_localised.or(self.experimental_effect),
            modifiers: self.modifiers.into_iter().map(|m| m.into()).collect(),
        }
    }
}

impl Into<state::Modifier> for event::LoadoutModuleEngineeringModifier {
    fn into(self) -> state::Modifier {
        state::Modifier {
            label: self.label,
            value: self.value.unwrap_or_default(),
            original_value: self.original_value.unwrap_or_default(),
            less_is_good: self.less_is_good.unwrap_or_default(),
        }
    }
}

impl Into<state::EngineerProgress> for event::EngineerProgress {
    fn into(self) -> state::EngineerProgress {
        state::EngineerProgress {
            engineers: self.engineers
                .unwrap_or_default()
                .into_iter()
                .map(|e| e.into())
                .collect(),
        }
    }
}

impl Into<state::Engineer> for event::EngineerProgressEngineer {
    fn into(self) -> state::Engineer {
        state::Engineer {
            engineer: self.engineer,
            engineer_id: self.engineer_id,
            progress: self.progress,
            rank_progress: self.rank_progress.unwrap_or_default(),
            rank: self.rank.unwrap_or_default(),
        }
    }
}