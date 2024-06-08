use crate::components::notifications::Notifications;
use crate::utils::ext_state::AppState;
use leptos::*;
use url::Url;

#[component]
pub fn Options() -> impl IntoView {
    provide_context(AppState::default());
    let state = use_context::<AppState>().unwrap();
    AppState::init_resource(state);

    let (url, set_url) = create_signal(state.blockmesh_url.get_untracked());

    let clear_action = create_action(move |_| async move {
        state.clear().await;
        AppState::set_success("Cache cleared".to_string(), state.success);
    });

    let save_action = create_action(move |_| async move {
        if url.get_untracked().is_empty() {
            AppState::set_error("URL is empty".to_string(), state.error);
            return;
        }
        let raw_url = url.get_untracked();
        let url = Url::parse(&url.get_untracked());
        match url {
            Err(error) => {
                AppState::set_error(format!("Invalid URL: {}", error), state.error);
                return;
            }
            Ok(url) => url,
        };
        state.blockmesh_url.update(|v| *v = raw_url.clone());
        set_url.update(|v| *v = raw_url.clone());
        AppState::store_blockmesh_url(raw_url).await;
        AppState::set_success("URL saved".to_string(), state.success);
    });

    view! {
        <Notifications/>
        <form on:submit=|ev| ev.prevent_default()>
            <div class="bg-gray-700 flex justify-center items-center">
                <div class="bg-gray-800 p-8 shadow-md w-full">
                    <p class="text-white">Options</p>
                    <div class="mb-4">
                        <label class="block text-white text-sm font-bold mb-2" for="url">
                            BlockMesh URL
                        </label>
                        <input
                            type="url"
                            required
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            placeholder=move || state.blockmesh_url.get()
                            name="url"
                            // prop:disabled=move || disabled.get()
                            on:keyup=move |ev: ev::KeyboardEvent| {
                                let val = event_target_value(&ev);
                                set_url.update(|v| *v = val);
                            }

                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                set_url.update(|v| *v = val);
                            }
                        />

                    </div>
                    <div class="flex items-center justify-between">
                        <button
                            class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                            on:click=move |_| save_action.dispatch(())
                        >
                            Submit
                        </button>
                    </div>

                </div>
            </div>
        </form>
        <form on:submit=|ev| ev.prevent_default()>
            <div class="bg-gray-700 flex justify-center items-center">
                <div class="bg-gray-800 border-white p-8 shadow-md w-full">
                    <div class="flex items-center justify-between">
                        <button
                            class="bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                            on:click=move |_| clear_action.dispatch(())
                        >
                            Reset Cache
                        </button>
                    </div>
                </div>
            </div>
        </form>
    }
}
