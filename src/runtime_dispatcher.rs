use std::sync::Arc;

use crate::event::Event;
use crate::event_log;
use crate::job_store::JobStore;

pub struct RuntimeDispatcher {
    job_store: Arc<JobStore>,
}

impl RuntimeDispatcher {
    pub fn new(
        job_store: Arc<JobStore>,
    ) -> Self {
        Self { job_store }
    }

    pub fn dispatch(
        &self,
        event: Event,
    ) {
        let _ =
            event_log::append_event(&event);

        self.job_store
            .apply_event(&event);
    }
}
