use crate::edsm;

pub struct StatusDetails {
    pub message: Box<str>,
    pub status: Box<str>,
}

impl From<edsm::ServerStatus> for StatusDetails {
    fn from(status: edsm::ServerStatus) -> Self {
        Self {
            message: status.message,
            status: status.r#type,
        }
    }
}