mod components {
    pub mod aboutme;
    pub mod bullet;
    pub mod footer;
    pub mod icons;
    pub mod navbar;
    pub mod timeline;
}
mod safehtml;

use crate::components::aboutme::AboutMe;
use crate::components::footer::Footer;
use crate::components::navbar::Navbar;

use yew::prelude::*;

#[function_component]
fn MainPage() -> Html {
    html! {
        <>
        // Navbar
        <Navbar />

        // Main content
        <AboutMe />

        // Footer
        <Footer />
        </>
    }
}

fn main() {
    yew::Renderer::<MainPage>::new().render();
}
