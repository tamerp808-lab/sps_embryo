use std::collections::HashMap;

use crate::event::{
    Event,
    EventType,
};

use crate::state::AppState;

pub type ReducerHandler =
    fn(
        &AppState,
        &Event,
    ) -> AppState;

#[derive(Debug)]
pub struct ReducerRegistry {

    reducers:
        HashMap<
            EventType,
            ReducerHandler,
        >,
}

impl ReducerRegistry {

    pub fn new() -> Self {

        Self {

            reducers:
                HashMap::new(),
        }
    }

    pub fn register(

        &mut self,

        event_type:
            EventType,

        handler:
            ReducerHandler,
    ) {

        self.reducers.insert(
            event_type,
            handler,
        );
    }

    pub fn dispatch(

        &self,

        state:
            &AppState,

        event:
            &Event,
    ) -> AppState {

        match self.reducers.get(
            &event.event_type
        ) {

            Some(handler) => {

                handler(
                    state,
                    event,
                )
            }

            None => {

                state.clone()
            }
        }
    }

    pub fn total_reducers(
        &self
    ) -> usize {

        self.reducers.len()
    }
}
