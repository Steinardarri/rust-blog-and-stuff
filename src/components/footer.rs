use yew::prelude::*;

#[function_component]
pub fn Footer() -> Html {
    html! {
        <div id="footer" class="pb-52 lg:pb-6 bg-nord6 dark:bg-nord0">
            <div class="flex justify-center items-center h-20">
                <a href="https://www.linkedin.com/in/steinardarri/" target="_blank" class="mr-6 lg:mr-4">
                    <img src="res/images/icons/linkedin.svg" width="36" height="24" />
                </a>
                <a href="https://github.com/steinardarri" target="_blank" class="mr-6 lg:mr-4">
                    <img src="res/images/icons/github.svg" width="24" height="24" />
                </a>
                <a href="mailto:Steinar@steinardth.xyz" class="mr-6 lg:mr-4">
                    <img src="res/images/icons/email.svg" width="24" height="24" />
                </a>
            </div>
            <p class="antialiased text-xl lg:text-md text-nord1 dark:text-nord5 text-center font-lg lg:font-medium pt-4 lg:pt-0 px-20">{
            "Ég bjó til þessa síðu með  "}
            <a href="https://yew.rs" target="_blank" class="italic lg:hover:underline decoration-nord8/70">{"Yew"}</a>
            {" vinnupallinum. Hún er í stöðugri þróun. Ekki hika við að skoða "}
            <a href="https://github.com/steinardarri/rust-blog-and-stuff" target="_blank" class="italic lg:hover:underline decoration-nord8/70">{"GitHub geymsluna"}</a>
            {" fyrir tæknilegar upplýsingar um síðuna."}
            </p>
        </div>
    }
}
