use std::sync::mpsc::Receiver;

use crate::Result;

pub struct App {
    exit: bool,
    ctrlc_rx: Option<Receiver<()>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            exit: false,
            ctrlc_rx: None,
        }
    }

    pub fn with_ctrlc_handler(mut self) -> Result<Self> {
        self.ctrlc_rx = Some(spawn_ctrlc_handler()?);
        Ok(self)
    }

    pub fn refresh(&self) {}

    pub fn exit(&self) -> bool {
        match &self.ctrlc_rx {
            Some(rx) => rx.try_recv().is_ok(),
            None => self.exit,
        }
    }
}

fn spawn_ctrlc_handler() -> Result<std::sync::mpsc::Receiver<()>> {
    let (ctrlc_tx, ctrlc_rx) = std::sync::mpsc::channel();
    ctrlc::try_set_handler(move || {
        ctrlc_tx
            .send(())
            .expect("Expected to send a signal on channel")
    })?;
    Ok(ctrlc_rx)
}
