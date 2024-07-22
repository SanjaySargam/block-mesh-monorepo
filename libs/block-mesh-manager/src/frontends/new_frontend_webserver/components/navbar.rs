use leptos::*;
use tailwind_fuse::*;

#[component]
pub fn Navbar(
    #[prop(into, optional)] class: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    let class = move || tw_merge!(class.get(), "flex flex-1 items-center gap-4 py-2.5");

    view! { <nav class=class>{children()}</nav> }
}

#[component]
pub fn NavbarSection(
    #[prop(into, optional)] class: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    let class = move || tw_merge!(class.get(), "flex items-center gap-3");

    view! { <div class=class>{children()}</div> }
}

#[component]
pub fn NavbarSpacer(#[prop(into, optional)] class: MaybeSignal<String>) -> impl IntoView {
    let class = move || tw_merge!(class.get(), "-ml-4 flex-1");

    view! { <div class=class aria-hidden="true"></div> }
}

#[component]
pub fn NavbarItem<F>(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(into)] aria_label: String,
    on_click: F,
    children: Children,
) -> impl IntoView
where
    F: Fn() + 'static,
{
    let root_class = move || tw_merge!(class.get(), "relative");

    let class = tw_join!(
        "cursor-default",
        // Base
        "relative flex min-w-0 items-center gap-3 rounded-lg p-2 text-left text-base/6 font-medium text-zinc-950 sm:text-sm/5",
        // Leading icon/icon-only
        "data-[slot=icon]:*:size-6 data-[slot=icon]:*:shrink-0 data-[slot=icon]:*:fill-zinc-500 sm:data-[slot=icon]:*:size-5",
        // Trailing icon (down chevron or similar)
        "data-[slot=icon]:last:[&:not(:nth-child(2))]:*:ml-auto data-[slot=icon]:last:[&:not(:nth-child(2))]:*:size-5 sm:data-[slot=icon]:last:[&:not(:nth-child(2))]:*:size-4",
        // Avatar
        "data-[slot=avatar]:*:-m-0.5 data-[slot=avatar]:*:size-7 data-[slot=avatar]:*:[--avatar-radius:theme(borderRadius.DEFAULT)] data-[slot=avatar]:*:[--ring-opacity:10%] sm:data-[slot=avatar]:*:size-6",
        // Hover
        "data-[hover]:bg-zinc-950/5 data-[slot=icon]:*:data-[hover]:fill-zinc-950",
        // Active
        "data-[active]:bg-zinc-950/5 data-[slot=icon]:*:data-[active]:fill-zinc-950",
        // Dark mode
        "dark:text-white dark:data-[slot=icon]:*:fill-zinc-400",
        "dark:data-[hover]:bg-white/5 dark:data-[slot=icon]:*:data-[hover]:fill-white",
        "dark:data-[active]:bg-white/5 dark:data-[slot=icon]:*:data-[active]:fill-white"
    );

    view! {
        <span class=root_class>
            <button class=class on:click=move |_| on_click() aria-label=aria_label>
                {children()}
            </button>
        </span>
    }
}

#[component]
pub fn NavbarLabel(
    #[prop(into, optional)] class: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    let class = move || tw_merge!(class.get(), "truncate");

    view! { <span class=class>{children()}</span> }
}
