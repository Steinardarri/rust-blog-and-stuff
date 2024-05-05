use yew::prelude::*;

#[function_component]
pub fn Navbar() -> Html {
    html! {
        <nav id="navbar is-primary" role="navigation" aria-label="main navigation" class="topbar visible">
            <ul class="flex flex-row h-min w-auto mx-2 topbar_links">
                <li class="w-28 mx-4">
                    <a href="/" class="flex flex-col items-center">
                        <img src="res/images/icons/home.svg" width="96" height="96" />
                        <span class="topbar_span">{"Heim"}</span>
                    </a>
                </li>
                <li class="w-28 mx-4">
                    <a href="/about" class="flex flex-col items-center">
                        <img src="res/images/icons/squareme.svg" width="96" height="96" />
                        <span class="topbar_span">{"Um mig"}</span>
                    </a>
                </li>
                <li class="w-28 mx-4">
                    <a href="/proj" class="flex flex-col items-center">
                        <img src="res/images/icons/projects.svg" width="96" height="96" />
                        <span class="topbar_span">{"Verkefni"}</span>
                    </a>
                </li>
                <li class="w-28 mx-4">
                    <a href="/blog" class="flex flex-col items-center">
                        <img src="res/images/icons/blog.svg" width="96" height="96" />
                        <span class="topbar_span">{"Blogg"}</span>
                    </a>
                </li>
            </ul>
        </nav>
    }
}
