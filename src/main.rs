use std::{io::Write, sync::mpsc};
use windows::{
    Foundation::{EventRegistrationToken, TypedEventHandler},
    Media::Control::{
        GlobalSystemMediaTransportControlsSession, GlobalSystemMediaTransportControlsSessionManager,
    },
};

/// See README.md for instructions.
fn main() -> windows::core::Result<()> {
    let (tx, rx) = mpsc::channel();

    let manager: GlobalSystemMediaTransportControlsSessionManager =
        GlobalSystemMediaTransportControlsSessionManager::RequestAsync()?.get()?;
    let tokens: Vec<(
        GlobalSystemMediaTransportControlsSession,
        EventRegistrationToken,
    )> = manager
        .GetSessions()?
        .into_iter()
        .filter_map(move |session: GlobalSystemMediaTransportControlsSession| {
            let tx = tx.clone();
            session
                .PlaybackInfoChanged(TypedEventHandler::new(move |_, _| {
                    tx.send(()).ok();
                    Ok(())
                }))
                .map(move |token| (session, token))
                .ok()
        })
        .collect();
    assert!(!tokens.is_empty());

    rx.recv().ok();
    for (session, token) in tokens {
        // this might fail if a session got removed
        session.RemovePlaybackInfoChanged(token).ok();
    }
    log_msg(&format!(
        "{} - Got event\n",
        time::OffsetDateTime::now_utc()
    ));

    Ok(())
}

/// Logs `message` to stdout and `test.log`
fn log_msg(message: &str) {
    std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("test.log")
        .unwrap()
        .write(message.as_bytes())
        .unwrap();
    println!("{}", message);
}
