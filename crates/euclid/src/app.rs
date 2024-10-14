use std::sync::RwLock;

use handlebars::Handlebars;
use sqlx::PgPool;

use crate::error::Error;

pub struct App {
    pub conn_pool: PgPool,
    pub handlebars: RwLock<Handlebars<'static>>,
}

impl App {
    pub fn new(conn_pool: PgPool) -> Result<Self, Error> {
        Ok(Self {
            conn_pool,
            handlebars: RwLock::new(Self::handlebars()?),
        })
    }

    fn handlebars() -> Result<Handlebars<'static>, Error> {
        let mut handlebars = Handlebars::new();
        Self::register_handlebar_templates(&mut handlebars)?;

        Ok(handlebars)
    }

    pub fn reload_handlebars(&self) -> Result<(), Error> {
        let mut handlebars = self
            .handlebars
            .write()
            .expect("RwLock poisoning for handlebars");
        handlebars.clear_templates();
        Self::register_handlebar_templates(&mut handlebars)?;

        Ok(())
    }

    fn register_handlebar_templates<'a>(handlebars: &mut Handlebars<'a>) -> Result<(), Error> {
        handlebars.register_template_file("base", "assets/templates/base.handlebars")?;
        handlebars.register_template_file("page", "assets/templates/page.handlebars")?;
        handlebars.register_template_file("not-found", "assets/templates/not-found.handlebars")?;

        Ok(())
    }
}
