use yew::prelude::*;

use crate::data::{get_portfolio, Portfolio};

use crate::AppContext;

#[function_component(Aboutme)]
pub fn aboutme() -> Html {
    let app_context: AppContext = use_context::<AppContext>().expect("No AppContext found!");

    fn handle_aboutme_content(app_context: AppContext) -> Html {
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
                let portfolio: Portfolio = get_portfolio();
                let about_me_list = portfolio.about;
                html! {
                <>
                    <ul class="item-list">
                        {
                            about_me_list.into_iter().map(|about| {
                                html!{<li>{ about }</li>}
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
        <h2>{ "About me" }</h2>
        <div class="aboutme">
            <div class="aboutme__description">
                <div class="aboutme__description__text">{ handle_aboutme_content(app_context) }</div>
            </div>
        </div>
    </>
    }
}
