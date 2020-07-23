use convey::{human, json, Render};
use serde::ser::Serialize;

pub struct ResponseRender<T: Serialize>(pub T);

impl<T: Serialize> Render for ResponseRender<T> {
    fn render_for_humans(&self, fmt: &mut human::Formatter) -> Result<(), convey::Error> {
        fmt.write(serde_json::to_vec(&self.0)?)?;
        Ok(())
    }

    fn render_json(&self, fmt: &mut json::Formatter) -> Result<(), convey::Error> {
        fmt.write(&self.0)?;
        Ok(())
    }
}
