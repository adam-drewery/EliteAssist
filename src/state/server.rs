use crate::edsm;

pub struct Status {
    pub message: Box<str>,
    pub status: Box<str>,
}

impl From<edsm::ServerStatus> for Status {
    fn from(status: edsm::ServerStatus) -> Self {
        Self {
            message: status.message,
            status: status.r#type,
        }
    }
}