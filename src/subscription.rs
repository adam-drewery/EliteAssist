use crate::state::{State, DataSource};
use iced::Subscription;
use crate::message::{Message, Query};

#[cfg(test)]
mod test;
mod hotkey;

#[cfg(not(feature = "mock_events"))]
mod journal;

#[cfg(not(feature = "mock_events"))]
pub fn subscription(state: &State) -> Subscription<Message> {
    match state.data_source {
        DataSource::Unselected => Subscription::run(stream_edsm_server_status),
        DataSource::Local => {
            if !state.journal_loaded {
                Subscription::batch(vec![
                    Subscription::run(journal::stream_history),
                    Subscription::run(stream_edsm_server_status),
                ])
            } else {
                Subscription::batch(vec![
                    Subscription::run(journal::stream_journal),
                    Subscription::run(journal::stream_status),
                    Subscription::run(journal::stream_backpack),
                    Subscription::run(journal::stream_cargo),
                    Subscription::run(journal::stream_ship_locker),
                    Subscription::run(journal::stream_market),
                    Subscription::run(journal::stream_navroute),
                    Subscription::run(hotkey::stream),
                    Subscription::run(stream_edsm_server_status),
                ])
            }
        }
        DataSource::Capi => {
            if !state.journal_loaded {
                if state.capi_enabled {
                    Subscription::batch(vec![
                        Subscription::run(journal::stream_capi_history),
                        Subscription::run(stream_edsm_server_status),
                    ])
                } else {
                    Subscription::run(stream_edsm_server_status)
                }
            } else {
                Subscription::batch(vec![
                    Subscription::run(hotkey::stream),
                    Subscription::run(stream_edsm_server_status),
                ])
            }
        }
    }
}

#[cfg(not(feature = "mock_events"))]
fn stream_edsm_server_status() -> impl iced::futures::Stream<Item = Message> {
    use tokio::sync::mpsc;
    use tokio_stream::wrappers::ReceiverStream;
    use std::time::Duration;

    let (sender, receiver) = mpsc::channel(8);

    tokio::spawn(async move {
        use crate::edsm::EdsmClient;
        use log::error;
        let client = EdsmClient::default();
        let mut interval = tokio::time::interval(Duration::from_secs(30));
        loop {
            interval.tick().await;
            match client.get_elite_server_status().await {
                Ok(status) => {
                    if sender.send(Message::Query(Query::EdsmServerStatus(status))).await.is_err() { break; }
                }
                Err(e) => {
                    error!("EDSM status poll failed: {}", e);
                }
            }
        }
    });

    ReceiverStream::new(receiver)
}

#[cfg(feature = "mock_events")]
mod example_data;

#[cfg(feature = "mock_events")]
pub fn subscription(_state: &State) -> Subscription<Message> {
    Subscription::run(example_data::stream)
}
