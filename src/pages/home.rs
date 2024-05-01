use yew::prelude::*;

#[function_component]
pub fn Home() -> Html {
    html! {
        <div id="home" class="flex flex-col justify-center bg-gradient-to-b from-fuchsia-300 to-violet-300 dark:from-fuchsia-600 dark:to-violet-700 pl-12 lg:pl-44 pr-20 w-full h-full">
            <h1 class="manual_h1">{ "Home" }</h1>
        </div>
    }
}
