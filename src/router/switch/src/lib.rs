use yew::prelude::*;
use route::Route;

use home::Home;
use articles_home::ArticlesHome;
use get_started_home::GetStartedHome;


// #[function_component(Secure)]
// fn secure() -> Html {
//     let history = use_history().unwrap();

//     let onclick = Callback::once(move |_| history.push(Route::Home));
//     html! {
//         <div>
//             <h1>{ "Secure" }</h1>
//             <button {onclick}>{ "Go Home" }</button>
//         </div>
//     }
// }


pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home/>
        },
        Route::Articles => html! {
            <ArticlesHome/>
        },
        Route::GetStartedHome => html! {
            <GetStartedHome/>
            // <div>{ "Get started home" }</div>
        },
        Route::Secure => html! {
            <div>{"SECURE"}</div>
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}