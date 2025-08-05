use crate::event;

impl Into<crate::state::Engineering> for event::LoadoutModuleEngineering {
    fn into(self) -> crate::state::Engineering {
        crate::state::Engineering {
            engineer: self.engineer.unwrap_or_default(),
            engineer_id: self.engineer_id,
            blueprint_id: self.blueprint_id,
            blueprint_name: self.blueprint_name.split('_').skip(1).next().unwrap().to_string(),
            level: self.level,
            quality: self.quality,
            experimental_effect: self.experimental_effect_localised.or(self.experimental_effect),
            modifiers: self.modifiers.into_iter().map(|m| m.into()).collect(),
        }
    }
}

impl Into<crate::state::Modifier> for event::LoadoutModuleEngineeringModifier {
    fn into(self) -> crate::state::Modifier {
        crate::state::Modifier {
            label: self.label,
            value: self.value.unwrap_or_default(),
            original_value: self.original_value.unwrap_or_default(),
            less_is_good: self.less_is_good.unwrap_or_default(),
        }
    }
}

impl Into<crate::state::EngineerProgress> for event::EngineerProgress {
    fn into(self) -> crate::state::EngineerProgress {
        crate::state::EngineerProgress {
            timestamp: self.timestamp,
            engineers: self.engineers
                .unwrap_or_default()
                .into_iter()
                .map(|e| e.into())
                .collect(),
        }
    }
}

impl Into<crate::state::Engineer> for event::EngineerProgressEngineer {
    fn into(self) -> crate::state::Engineer {
        crate::state::Engineer {
            engineer: self.engineer,
            engineer_id: self.engineer_id,
            progress: self.progress,
            rank_progress: self.rank_progress.unwrap_or_default(),
            rank: self.rank.unwrap_or_default(),
        }
    }
}