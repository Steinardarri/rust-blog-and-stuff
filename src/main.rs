use yew::prelude::*;
use yew_router::prelude::*;

mod components {
    pub mod footer;
    pub mod header;
}
mod pages {
    pub mod aboutme;
    pub mod home;
}

use crate::components::footer::Footer;
use crate::components::header::Navbar;

use crate::pages::aboutme::AboutMe;
use crate::pages::home::Home;

#[derive(Copy, Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    // Home
    AboutMe,
    #[at("/about")]
    // AboutMe,
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn scroll_to_top() {
    if let Some(window) = web_sys::window() {
        let mut options = web_sys::ScrollToOptions::new();
        options.top(0.0);
        window.scroll_with_scroll_to_options(&options);
    }
}

fn switch(routes: Route) -> Html {
    scroll_to_top();

    match routes {
        Route::Home => html! { <Home /> },
        Route::AboutMe => html! { <AboutMe /> },
        Route::NotFound => html! {
            <div id="notfound" class="flex flex-col justify-center bg-gradient-to-b from-nord10 to-nord7 dark:from-nord15 dark:to-nord10 pl-12 lg:pl-44 pr-20 w-full h-full">
                <h1 class="manual_h1 text-center">{ "404" }</h1>
            </div>
        },
    }
}

#[function_component]
fn MainPage() -> Html {
    html! {
        <>
        <Navbar />

        // Main content
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>

        <Footer />
        </>
    }
}

fn main() {
    yew::Renderer::<MainPage>::new().render();
}
