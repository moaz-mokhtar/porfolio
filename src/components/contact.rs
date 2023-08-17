use yew::prelude::*;

use crate::components::svg::{
    emojis::Mail,
    flags::{China, Egypt, England, Germany, Japan, Korea},
};
use crate::AppContext;

#[function_component(Contact)]
pub fn contact() -> Html {
    let app_context = use_context::<AppContext>().expect("No AppContext found!");

    fn handle_title(app_context: AppContext) -> &'static str {
        match app_context.language.current {
            "kr" => "편하게 연락해주세요!",
            "eng" | "de" | "jp" | _ => "Contact",
        }
    }

    fn handle_contact_description(app_context: AppContext) -> Html {
        match app_context.language.current {
            "de" => html! {
            <>
                <p>{ "Egal ob Du geschäftlich mit mir in Kontakt treten " }
                { "möchtest oder einfach mit mir abhängen möchtest, ich freue mich mit Dir zu quatschen." }</p>
                <p>{ "Meine Muttersprache ist Deutsch" } <Germany class="svg-text" />
                { ", aber ich kann auch Englisch" } <England class="svg-text" />
                { ", Japanisch" } <Japan class="svg-text" />
                { ", Koreanisch" } <Korea class="svg-text" />
                { " und lerne momentan Mandarin" } <China class="svg-text" /> { " und Arabisch" } <Egypt class="svg-text" />
                { ". Du kannst mich gerne in einer dieser Sprachen anschreiben." }</p>
                <p>{ "Am liebsten bevorzuge ich " } <a href="mailto:marc.maeurer@pm.me">{ "email" }<Mail class="svg-text" /></a></p>
            </>
            },
            "jp" => html! {
            <>
                <p>{ "母国語はドイツ語" } <Germany class="svg-text" />
                { "ですが、英語" } <England class="svg-text" />
                { "と日本語" } <Japan class="svg-text" />
                { "と韓国語" } <Korea class="svg-text" />
                { "も喋れます。そして、今中国語" } <China class="svg-text" /> { "とアラビア語" } <Egypt class="svg-text" />
                { "を勉強しています。" }
                { "私と仕事をしてみたい、または楽しく会話してみたい、と思った方はどんな言語でも気軽に連絡して下さい。" }</p>
                <p><a href="mailto:marc.maeurer@pm.me">{ "お問い合わせは" } <Mail class="svg-text" /></a></p>
            </>

            },
            "kr" => html! {
            <>
                <p>{ "저와 함께 이야기를 나누고 싶거나, 같이 일해 보고 싶다면 언제든지 연락해주세요. " }
                { "제 모국어는 독일어" } <Germany class="svg-text" />
                { "이지만 저는 영어" } <England class="svg-text" />
                { "와 일본어" } <Japan class="svg-text" />
                { ",  한국어" } <Korea class="svg-text" />
                { "도 가능합니다. 그뿐만 아니라 현재 중국어" } <China class="svg-text" /> { "와 아랍어" } <Egypt class="svg-text" />
                { "도 배우고 있습니다. 어떤 언어로든 저에게 부담없이 연락해주시면 됩니다." }</p>
                <p><a href="mailto:marc.maeurer.@pm.me">{ "문의처" } <Mail class="svg-text" /></a></p>
            </>

            },
            "eng" | _ => html! {
            <>
                // TODO 2023-08-17: Add icons
                <p>{ "I'm open for business, tasks, jobs, or casual conversation." } </p>
                <ul>
                    <li>
                        {"Email: "}
                        <a href="mailto:moaz.mokhtar@gmail.com">
                            // <Mail class="svg-text" />
                            { " email" }
                        </a>
                    </li>
                    <li>
                        {"LinkedIn: "}
                        <a href="https://www.linkedin.com/in/moazmoktar/">
                            // <Linkedin class="fa-linkedin" />
                            { " moazmoktar" } </a>
                    </li>
                    <li>
                        {"Github: "}<a href="https://github.com/moaz-mokhtar">
                        // <Github  />
                            { " moaz-mokhtar" }
                        </a>
                    </li>
                </ul>
            </>

            },
        }
    }

    html! {
    <>
        <h2 id="contact"> { handle_title(app_context.clone()) } </h2>
        <div class="contact margin-top">
            // <QuickChat class="contact__illustration"/>
            <div class="contact__paragraph"> { handle_contact_description(app_context) } </div>
        </div>
        </>
    }
}
