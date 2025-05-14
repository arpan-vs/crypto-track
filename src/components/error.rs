// src/components/error.rs
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ErrorProps {
    pub message: String,
    pub on_retry: Option<Callback<()>>,
}

#[function_component(Error)]
pub fn error(props: &ErrorProps) -> Html {
    let on_retry = {
        let on_retry = props.on_retry.clone();
        Callback::from(move |_| {
            if let Some(callback) = &on_retry {
                callback.emit(());
            }
        })
    };
    
    html! {
        <div class="error-container flex flex-col items-center justify-center bg-red-50 border border-red-200 rounded p-6 my-4">
            <div class="error-icon text-4xl mb-2">{"⚠️"}</div>
            <p class="error-message text-red-700 font-semibold mb-2">{&props.message}</p>
            {
                if props.on_retry.is_some() {
                    html! { <button class="px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700 transition" onclick={on_retry}>{"Retry"}</button> }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
