use yew::prelude::*;

struct App {
    campaigns: Vec<String>,
}

enum Msg {
    FetchCampaigns,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            campaigns: vec![],
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchCampaigns => {
                self.campaigns = vec![
                    "Campaign: Bantuan Bencana Alam - Rp 500,000,000 (Terkumpul: Rp 250,000,000)".into(),
                    "Campaign: Bantuan Pendidikan Anak Yatim - Rp 300,000,000 (Terkumpul: Rp 150,000,000)".into(),
                ];
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{ "Galang Dana" }</h1>
                <button onclick={ctx.link().callback(|_| Msg::FetchCampaigns)}>{ "Lihat Kampanye" }</button>
                <ul>
                    { for self.campaigns.iter().map(|campaign| html! { <li>{ campaign }</li> }) }
                </ul>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<App>::new().mount_to_body();
}
