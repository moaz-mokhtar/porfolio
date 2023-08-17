use yew::prelude::*;

use crate::AppContext;

#[function_component(Footer)]
pub fn footer() -> Html {
    let app_context: AppContext = use_context::<AppContext>().expect("No AppContext found!");

    fn handle_footer_content(app_context: AppContext) -> Html {
        let rust_link: Html = html! { <a href="https://www.rust-lang.org/" target="_blank" rel="noopener noreferrer">{ "Rust Programming Language" }</a> };
        let yew_link: Html = html! { <a href="https://yew.rs/" target="_blank" rel="noopener noreferrer">{ "Yew Framework" }</a> };
        let wasm_link: Html = html! { <a href="https://webassembly.org/" target="_blank" rel="noopener noreferrer">{ "WebAssembly" }</a> };
        let orignal_repo: Html = html! { <a href="https://gitlab.com/marcempunkt/maeurerdev" target="_blank" rel="noopener noreferrer">{ "Marc's Portfolio repo" }</a> };

        match app_context.language.current {
            "de" => html! {
                    <>
                {  "Diese Website wurde im " } { yew_link } { " programmiert und zu " } { wasm_link } { " kompliert." }
                    </>
            },
            "jp" => html! {
            <>
                {  "このサイトは" } { yew_link } { "で作られて" } { wasm_link } { "にコンパイルされました。" }
                    </>
            },
            "kr" => html! {
            <>
                {  "이 사이트는 " } { yew_link } { "에서 제작되어 " } { wasm_link } { "에 편집되었습니다." }
                    </>
            },
            "eng" | _ => html! {
            <>
                {  "This website was created using " } {rust_link} {" and the " } { yew_link } { " then compiled to " } { wasm_link } { "." }<br/>
				{ "I did use template of: " } { orignal_repo }
                    </>
            },
        }
    }

    html! {
    <footer>
        { handle_footer_content(app_context) }
    </footer>
    }
}
