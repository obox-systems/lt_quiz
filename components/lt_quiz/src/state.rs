use std::rc::Rc;

use lt_quiz_core::ir;

use crate::{db, toml, Result};

pub(crate) struct RawState {
    pub(crate) config: ir::Config,
    pub(crate) db: db::Sqlite,
    pub(crate) cache: std::cell::RefCell<anymap::AnyMap>,
}

#[derive(Clone)]
pub(crate) struct State {
    raw: Rc<RawState>,
}

impl State {
    pub(crate) fn new(config: ir::Config, db: db::Sqlite) -> Self {
        Self { raw: RawState { config, db, cache: anymap::AnyMap::new().into() }.into() }
    }

    pub(crate) fn questions(
        &self,
        has_tags: Vec<String>,
        no_tags: Vec<String>,
    ) -> Result<Vec<toml::Question>> {
        use lt_quiz_core::traits::Database as _;

        let mut cache = self.raw.cache.borrow_mut();
        match cache.get::<Vec<toml::Question>>() {
            Some(questions) => Ok(questions.clone()),
            None => {
                let questions = self.raw.db.find_questions(has_tags, no_tags)?;
                cache.insert(questions.clone());
                Ok(questions)
            }
        }
    }
}

impl std::ops::Deref for State {
    type Target = RawState;

    fn deref(&self) -> &RawState {
        &self.raw
    }
}
