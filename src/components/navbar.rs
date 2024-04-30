use yew::prelude::*;

#[function_component]
pub fn Navbar() -> Html {
    html! {
        <>
        // Sidebar (desktop)
        <aside id="sidebar" class="group sidebar invisible lg:visible">
            <ul class="sidebar_links">
                <li>
                    <a href="#aboutme" class="flex items-center">
                        <img src="res/images/icons/aboutme.svg"/>
                        <span class="sidebar_span hidden group-hover:block">{"About me"}</span>
                    </a>
                </li>
                <li>
                    <a href="#profexper" class="flex items-center">
                        <img src="res/images/icons/profexper.svg"/>
                        <span class="sidebar_span hidden group-hover:block">{"Professional experience"}</span>
                    </a>
                </li>
                <li>
                    <a href="#educ" class="flex items-center">
                        <img src="res/images/icons/educ.svg"/>
                        <span class="sidebar_span hidden group-hover:block">{"Education"}</span>
                    </a>
                </li>
/*                 <li>
                    <a href="#projects" class="flex items-center">
                        <img src="res/images/icons/projects.svg"/>
                        <span class="sidebar_span hidden group-hover:block">{"Projects"}</span>
                    </a>
                </li>
                <li>
                    <a href="#techskills" class="flex items-center">
                        <img src="res/images/icons/techskills.svg"/>
                        <span class="sidebar_span hidden group-hover:block">{"Technical skills"}</span>
                    </a>
                </li> */
            </ul>
        </aside>

        // Bottombar (mobile)
        <aside id="bottombar" class="group bottombar visible lg:invisible">
            <ul class="flex flex-row h-max w-auto mx-2">
                <li class="w-28 mx-4">
                    <a href="#aboutme" class="flex flex-col items-center">
                        <img src="res/images/icons/aboutme.svg"/>
                        <span class="bottombar_span">{"About me"}</span>
                    </a>
                </li>
                <li class="w-28 mx-4">
                    <a href="#profexper" class="flex flex-col items-center">
                        <img src="res/images/icons/profexper.svg"/>
                        <span class="bottombar_span">{"Experience"}</span>
                    </a>
                </li>
                <li class="w-28 mx-4">
                    <a href="#educ" class="flex flex-col items-center">
                        <img src="res/images/icons/educ.svg"/>
                        <span class="bottombar_span">{"Education"}</span>
                    </a>
                </li>
/*                 <li class="w-28 mx-4">
                    <a href="#projects" class="flex flex-col items-center">
                        <img src="res/images/icons/projects.svg"/>
                        <span class="bottombar_span">{"Projects"}</span>
                    </a>
                </li>
                <li class="w-28 mx-4">
                    <a href="#techskills" class="flex flex-col items-center">
                        <img src="res/images/icons/techskills.svg"/>
                        <span class="bottombar_span">{"Skills"}</span>
                    </a>
                </li> */
            </ul>
        </aside>
        </>
    }
}
