use yew::prelude::*;

use crate::components::svg::emojis::Handshake;

#[function_component(Landing)]
pub fn landing() -> Html {
    html! {
    <>
        <p class="landing">
            <i > {"In the name of Allah, the Most Gracious, the Most Merciful"}
			</i>
        </p>

        <h1 class="landing">
        { "Hi "} <Handshake class="wave" /> {"! This is Moaz" }  <br />
        { "I'm a Rust Software Developer" }
        </h1>

		// // TODO 2023-08-16: Change js script to more convenient script
        // <script src={ "./vendor/lottie_player_v157_bundle.js" }></script>

        // <lottie-player
        // class="lottie-landing"
        // // This Lottiefile is licensed by CC so he must be attributed
        // // title="Jignesh Gajjar @LottieFiles"
        // src="./assets/lottie/coder_with_coffee_mug.json"
        // background="transparent"
        // speed="1"
        // loop={ true }
        // autoplay={ true } >
        // </lottie-player>
    </>
    }
}
