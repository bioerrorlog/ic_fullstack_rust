use anyhow::{
    Result,
    Context,
};
use candid::{Principal, Encode, Decode};
use ic_agent::agent::AgentError;
use yew::prelude::*;

use ic_fullstack_rust_assets::{
    get_agent,
    Network,
};

async fn call_greet(name: &str) -> Result<String> {
    let caniter_id = Principal::from_text("aaaaa-aa")?;
    let agent = get_agent(Network::Local).await?;
    let res = agent.query(&caniter_id, "greet")
        .with_arg(Encode!()?)
        .call()
        .await?;

    Ok(String::from(name))
}

#[function_component(Model)]
fn model() -> Html {
    let state = use_state(|| 0);
    // let greet_state = use_state(|| "default".to_string());

    let inc_counter = {
        println!("indside inc_counter");
        let state = state.clone();
        Callback::from(move |_| state.set(*state + 1))
    };

    let dec_counter = {
        let state = state.clone();
        Callback::from(move |_| state.set(*state - 1))
    };

    html! {
        <div>
            <p>{ *state }</p>
            <button onclick={inc_counter}>{ "+1" }</button>
            <button onclick={dec_counter}>{ "-1" }</button>
        </div>
    }
}

fn main() {
    yew::start_app::<Model>();
}
