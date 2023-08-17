use crate::data::{get_portfolio, Portfolio, Project};
use crate::AppContext;
use yew::prelude::*;

#[function_component(Projects)]
pub fn projects() -> Html {
    let app_context: AppContext = use_context::<AppContext>().expect("No AppContext found!");

    fn handle_projects_content(app_context: AppContext) -> Html {
        match app_context.language.current {
            "de" => html! {
            <>

            </>
            },
            "jp" => html! {
            <>

            </>
            },
            "kr" => html! {
            <>

            </>
            },
            "eng" | _ => {
                let projects: Vec<Project> = get_portfolio().projects;
                let projects_names = projects.iter().map(|project| project.name.clone());
                html! {
                <>
                    <ul class="item-list">
                        {
                            projects_names.into_iter().map(|name| {
                                html!{<li>{ name }</li>}
                            }).collect::<Html>()
                        }
                    </ul>
                </>
                }
            }
        }
    }

    html! {
    <>
        <h2>{ "Projects" }</h2>
            <div class="projects">
                <div class="projects__description">
                <div class="projects__description__text">{ handle_projects_content(app_context) }</div>
            </div>
        </div>
    </>
    }
}
