use yew::prelude::*;

#[function_component]
pub fn AboutNavbar() -> Html {
    html! {
        <>
        // Sidebar (desktop)
        <aside id="sidebar" class="group sidebar invisible lg:visible">
            <ul class="sidebar_links">
                <li>
                    <a href="#aboutme" class="flex items-center">
                        <img src="res/images/icons/aboutme.svg" width="48" height="48" />
                    </a>
                </li>
                <li>
                    <a href="#profexper" class="flex items-center">
                        <img src="res/images/icons/profexper.svg" width="48" height="48" />
                    </a>
                </li>
                <li>
                    <a href="#educ" class="flex items-center">
                        <img src="res/images/icons/educ.svg" width="48" height="48" />
                    </a>
                </li>
                /*<li>
                    <a href="#techskills" class="flex items-center">
                        <img src="res/images/icons/techskills.svg" width="48" height="48" />
                    </a>
                </li>*/
            </ul>
        </aside>

        // Bottombar (mobile)
        <aside id="bottombar" class="group bottombar visible lg:invisible">
            <ul class="flex flex-row h-max w-auto mx-2">
                <li class="w-28 mx-4">
                    <a href="#aboutme" class="flex flex-col items-center">
                        <img src="res/images/icons/aboutme.svg" width="48" height="48" />
                    </a>
                </li>
                <li class="w-28 mx-4">
                    <a href="#profexper" class="flex flex-col items-center">
                        <img src="res/images/icons/profexper.svg" width="48" height="48" />
                    </a>
                </li>
                <li class="w-28 mx-4">
                    <a href="#educ" class="flex flex-col items-center">
                        <img src="res/images/icons/educ.svg" width="48" height="48" />
                    </a>
                </li>
                /*<li class="w-28 mx-4">
                    <a href="#techskills" class="flex flex-col items-center">
                        <img src="res/images/icons/techskills.svg" width="48" height="48" />
                    </a>
                </li>*/
            </ul>
        </aside>
        </>
    }
}
