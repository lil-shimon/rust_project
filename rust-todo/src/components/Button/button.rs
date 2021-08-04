use yew::prelude::*;

pub struct Button {
    title: String,
    link: ComponentLink<Self>,
};

pub enum Msg {
    Clicked,
};

// [prop_or_default] -> in case props don't indicate, type default value is used
pub struct Props {
    #[prop_or_default]
    pub title: String,
}

impl Component for Button {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Button {
            title: props.title,
            link,
        }
    }

    fn view(&self) -> Html {
        html! {
            <button>
            { &self.title }
            </button>
        }
    }
}

