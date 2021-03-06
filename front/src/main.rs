use yew::prelude::*;

pub enum Msg {
    AddOne,
    SubstractOne,
}

pub struct Counter {
    pub value_1: i64,
    pub name: String,
}

impl Component for Counter {
    type Message = Msg;
    type Properties = ();

    //create a context
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value_1: 0,
            name: String::from("Javier"),
        }
    }

    //update values
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value_1 += 1;
                self.name = String::from("jga+1");
                true
            }
            Msg::SubstractOne => {
                self.value_1 -= 1;
                self.name = String::from("jga-1");
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let header = "Framework Yew";
        let h2 = "Img";

        html! {
            <main>
                <div>
                    <h1 class="color">{ header }</h1>
                </div>
                <div>
                    <p>{ "Value : " } { self.value_1 }</p>
                    <p>{ "Name : " } { &self.name } </p>
                    <button onclick={link.callback(|_| Msg::AddOne)}>{ "Add +1" }</button>
                    <button onclick={link.callback(|_| Msg::SubstractOne)}>{ "Substract -1" }</button>
                </div>
                <div>
                    <h2 class="color">{ h2 }</h2>
                    <img src="static/logo.png"/>
                </div>
            </main>
        }
    }
}

fn main() {
    //start app
    yew::start_app::<Counter>();
}
