use yew::prelude::*;

use crate::atoms::{button::Button, input::Input};

#[function_component(Login)]
pub fn login() -> Html {
	html! {
		<div class="d:flex h:100vh m:8 ai:center jc:center">
			<div class="b:1|solid|gray-80 m:14 r:10 px:16 py:8 min-w:360">
				<div class="t:center mb:16">
					<h1 class="mb:8 f:24">{"Sign In"}</h1>
					<p class="">{"Use your Whole Account"}</p>
				</div>

				<Input
					label="ID"
					placeholder="XYZ123456789"
					r#type="text"
				/>
				<Button text="Login" />
			</div>
		</div>
	}
}
