use crate::AppContext;
use yew::prelude::*;

#[function_component(Skills)]
pub fn skills() -> Html {
    let _app_context = use_context::<AppContext>().expect("No AppContext not found!");

    fn _handle_title(app_context: AppContext) -> &'static str {
        match app_context.language.current {
            "de" => "Meine Skills",
            "jp" => "スキル",
            "kr" => "저의 스킬",
            "eng" | _ => "My Skills",
        }
    }

    html! {

      <>
        <h2>{ "Skills" } </h2>

        <div class="skills">
        // TODO 2023-08-16: Add Extra icons
          <div class="skills__item">
            <img src="./assets/images/tech-icons/rust.png" alt="Rust" loading="lazy"/>
            <span class="skills__tooltip">{ "Rust" }</span>
          </div>
          <div class="skills__item">
            <img src="./assets/images/tech-icons/linux.png" alt="GNU/Linux" loading="lazy"/>
            <span class="skills__tooltip">{ "GNU/Linux" }</span>
          </div>
          <div class="skills__item">
            <img src="./assets/images/tech-icons/git.png" alt="Git" loading="lazy"/>
            <span class="skills__tooltip">{ "Git" }</span>
          </div>
          <div class="skills__item">
            <img src="./assets/images/tech-icons/docker.png" alt="Docker" loading="lazy"/>
            <span class="skills__tooltip">{ "Docker" }</span>
          </div>

          <div class="skills__item">
            <img src="./assets/images/tech-icons/wasm.png" alt="WebAssembly" loading="lazy"/>
            <span class="skills__tooltip">{ "WebAssembly" }</span>
          </div>

          <div class="skills__item">
            <img alt="VS Code" loading="lazy"/>
            <span class="skills__tooltip">{ "VS Code" }</span>
          </div>


          <div class="skills__item">
            <img alt="Postgresql" loading="lazy"/>
            <span class="skills__tooltip">{ "Postgresql" }</span>
          </div>

        </div>
      </>
    }
}
