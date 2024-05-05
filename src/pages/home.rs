use yew::prelude::*;

#[function_component]
pub fn Home() -> Html {
    html! {
        <div id="home" class="flex flex-col justify-center bg-gradient-to-b from-nord10 to-nord7 dark:from-nord15 dark:to-nord10 pl-12 lg:pl-44 pr-20 w-full h-full">
            <h1 class="manual_h1">{ "Home" }</h1>
        </div>
    }
}
