use std::ops::Not;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct TimelineStepProps {
	pub step: Option<i8>,
	#[prop_or(false)]
	pub last: bool,
	pub children: Children,
}

#[function_component]
pub fn TimelineStep(props: &TimelineStepProps) -> Html {
	html! {
		<li class={classes!(
			String::from("flex rel gap:24"),
			String::from("mb:36px"),
			// Vertical line - not necessary for the last Step
			&props.last
				.not()
				.then(|| Some(String::from("{content:'';bg:gray-86;abs;w:1;h:100%;left:0;z-index:0;ml:14px;mt:36px}::after")))
		)}>
			<p class="b:1|solid|gray-86 bg:white p:2 h:28 w:28 line-height:1.4rem t:center round f:semibold f:12">
				{props.step.unwrap()}
			</p>
			<div class="mt:4">
				{props.children.clone()}
			</div>
		</li>
	}
}