use serde::Deserialize;

#[derive(Deserialize)]
pub struct Cqc {

    #[serde(rename = "CQC_Credits_Earned")]
    pub cqc_credits_earned: u64,

    #[serde(rename = "CQC_Time_Played")]
    pub cqc_time_played: u64,

    #[serde(rename = "CQC_KD")]
    pub cqc_kd: f64,

    #[serde(rename = "CQC_Kills")]
    pub cqc_kills: u64,

    #[serde(rename = "CQC_WL")]
    pub cqc_wl: f64,
}