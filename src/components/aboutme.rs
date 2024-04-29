use yew::prelude::*;

// use crate::components::icons::Icons;
// use crate::safehtml::SafeHtml;

use crate::components::timeline::{Entry, Timeline};
// use comrak::{markdown_to_html, Options};

#[function_component]
pub fn AboutMe() -> Html {
    // Markdown render settings
    // let mut options = Options::default();
    // options.render.unsafe_ = true;

    // Get content
    let content_profexper = include_str!("./../../res/content/profexper.yaml");
    let content_educ = include_str!("./../../res/content/educ.yaml");
    // let content_projects =
    //     markdown_to_html(include_str!("./../../res/content/projects.md"), &options);

    // Iterate over each timeline entry and create TimelineProps
    // Professional experience
    let content_profexper_entries: Vec<Entry> = serde_yaml::from_str(content_profexper).unwrap();
    let content_profexper_timeline: Vec<Html> = content_profexper_entries
        .iter()
        .map(|entry| {
            html! {
                <Timeline
                    title={entry.title.clone()}
                    place={entry.place.clone()}
                    place_url={entry.place_url.clone()}
                    time={entry.time.clone()}
                    skills={entry.skills.clone()}
                    color={entry.color.clone()}
                    content={entry.content.clone()}
                />
            }
        })
        .collect();
    // Education
    let content_educ_entries: Vec<Entry> = serde_yaml::from_str(content_educ).unwrap();
    let content_educ_timeline: Vec<Html> = content_educ_entries
        .iter()
        .map(|entry| {
            html! {
                <Timeline
                    title={entry.title.clone()}
                    place={entry.place.clone()}
                    place_url={entry.place_url.clone()}
                    time={entry.time.clone()}
                    skills={entry.skills.clone()}
                    color={entry.color.clone()}
                    content={entry.content.clone()}
                />
            }
        })
        .collect();

    html! {
     <>
     // About Me
     <div id="aboutme" class="flex flex-col justify-center bg-gradient-to-b from-fuchsia-300
     to-violet-300 dark:from-fuchsia-600 dark:to-violet-700 pl-12 lg:pl-44 pr-20 w-full h-full">
         <h1 class="manual_h1">{"Steinar Darri Þorgilsson"}</h1>
         <div class="flex mb-6">
             <a id="linkedin" href="https://www.linkedin.com/in/steinardarri/" target="_blank" class="mr-6 lg:mr-4">
                 <img src="res/images/icons/linkedin.svg"/>
             </a>
             <a id="github" href="https://github.com/steinardarri" target="_blank" class="mr-6 lg:mr-4">
                 <img src="res/images/icons/github.svg"/>
             </a>
             <a id="email" href="mailto:steinardth@gmail.com" class="mr-6 lg:mr-4">
                 <img src="res/images/icons/email.svg"/>
             </a>
         </div>
         <span class="type-aboutme font-mono antialiased text-5xl lg:text-3xl font-medium
         tracking-normal text-wrap text-stone-600 dark:text-neutral-300 leading-tight lg:leading-snug"></span>
     </div>

     // Professional experience
    <div id="profexper" class="bg-gradient-to-b from-violet-300 to-green-100 dark:from-violet-700 dark:to-emerald-900 pt-28 pb-10 pl-12 lg:pl-44 pr-20">
         <h1 class="text-8xl lg:text-7xl manual_h1" data-aos="fade">{"Starfsreynsla"}</h1>
         { for content_profexper_timeline.iter().cloned() }
     </div>

     // Education
     <div id="educ" class="bg-gradient-to-b from-green-100 to-teal-200 dark:from-emerald-900 dark:to-teal-800 pt-28 pb-14 pl-12 lg:pl-44 pr-20">
         <h1 class="text-8xl lg:text-7xl manual_h1" data-aos="fade">{"Menntun"}</h1>
         { for content_educ_timeline.iter().cloned() }
     </div>

    /*  // Projects
     <div id="projects" class="bg-gradient-to-b from-teal-200 to-cyan-200 dark:from-teal-800 dark:to-cyan-900
     pt-28 pb-14 pl-12 lg:pl-44 pr-20">
         <h1 class="text-8xl lg:text-7xl manual_h1" data-aos="fade">{"Projects"}</h1>
         <SafeHtml html={content_projects}/>
     </div>

     // Technical skills
     <div id="techskills" class="bg-gradient-to-b from-cyan-200 to-indigo-300 dark:from-cyan-900 dark:to-indigo-900 pt-28 pb-14 pl-12 lg:pl-44 pr-20">
         <h1 class="text-8xl lg:text-7xl manual_h1" data-aos="fade">{"Technical skills"}</h1>
         <div class="flex flex-col items-center">
             <span class="text-gray-700 dark:text-stone-200 text-opacity-90 dark:text-opacity-90 text-7xl lg:text-5xl font-semibold mb-6 lg:mb-4 text-center" data-aos="fade">{"Programming languages"}</span>
             <Icons icons={vec![
                 "matlab".to_string(),
                 "python".to_string(),
                 "r".to_string(),
                 "sql".to_string(),
                 "stata".to_string()
             ]}/>
             <span class="text-gray-700 dark:text-stone-200 text-opacity-90 dark:text-opacity-90 text-7xl lg:text-5xl font-semibold mt-10 lg:mt-6 mb-6 lg:mb-4 text-center" data-aos="fade">{"Frameworks & libraries"}</span>
             <Icons icons={vec![
                 "arrow".to_string(),
                 "dash".to_string(),
                 "detectron".to_string(),
                 "gradio".to_string(),
                 "huggingface".to_string(),
                 "kafka".to_string(),
                 "llamaindex".to_string(),
                 "pytorch".to_string(),
                 "scikitlearn".to_string(),
                 "shiny".to_string(),
                 "spark".to_string(),
                 "yew".to_string()
             ]}/>
             <span class="text-gray-700 dark:text-stone-200 text-opacity-90 dark:text-opacity-90 text-7xl lg:text-5xl font-semibold mt-10 lg:mt-6 mb-6 lg:mb-4 text-center" data-aos="fade">{"Tools"}</span>
             <Icons icons={vec![
                 "atlassian".to_string(),
                 "css".to_string(),
                 "earthengine".to_string(),
                 "git".to_string(),
                 "html".to_string(),
                 "informatica".to_string(),
                 "powerbi".to_string(),
                 "oracledb".to_string(),
                 "postgresql".to_string(),
                 "qgis".to_string(),
                 "tailwind".to_string()
             ]}/>
         </div>
     </div> */
     </>
     }
}
