// reduce import statement
// ->
// pub use html::Component;
// pub use html::Env;
// pub use html::Href;
// pub use html::Html;
// pub use html::ChangeData;
// pub use html::InputData;
// pub use html::Renderable;
// pub use html::ShouldRender;
// pub use app::App;
// pub use callback::Callback;

use yew::prelude::*;

pub struct App {}

pub enum Msg {}

impl Component for App {
    // Properties is like props...?
    // but a bit different
    // should use #[derive(Properties)]
    type Properties = ();

    type Message = Msg;

    // ComponentLink -> Link to component's scope for creating callbacks.
    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    // ShouldRender -> This type indicates that component should be rendered again.
    // [bool]
    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div></div>
        }
    }
}
