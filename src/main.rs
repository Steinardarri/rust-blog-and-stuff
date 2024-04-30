mod components {
    pub mod footer;
    pub mod navbar;
}
mod pages {
    pub mod aboutme;
}

use crate::components::footer::Footer;
use crate::components::navbar::Navbar;
use crate::pages::aboutme::AboutMe;

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
