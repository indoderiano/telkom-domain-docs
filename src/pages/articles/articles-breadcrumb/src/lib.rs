use yew::prelude::*;
use get_started_topics::{
    Topic,
    SubTopic,
};


#[derive(Properties, PartialEq, Debug)]
pub struct ArticlesBreadcrumbProps {
    #[prop_or(Topic::Home)]
    pub topic: Topic,
    #[prop_or(SubTopic::Home)]
    pub sub_topic: SubTopic,
}

pub struct ArticlesBreadcrumb {
    topic: Topic,
    sub_topic: SubTopic,
}

pub enum Msg {}

impl Component for ArticlesBreadcrumb {
    type Message = Msg;
    type Properties = ArticlesBreadcrumbProps;

    fn create(ctx: &Context<Self>) -> Self {
        // log::info!("This is log from articles breadcrumb");
        ArticlesBreadcrumb {
            topic: ctx.props().topic.to_owned(),
            sub_topic: ctx.props().sub_topic.to_owned(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        if self.topic != ctx.props().topic {
            self.topic = ctx.props().topic.to_owned();
            true
        } else if self.sub_topic != ctx.props().sub_topic {
            self.sub_topic = ctx.props().sub_topic.to_owned();
            true
        } else {
            false
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        match self.topic {
            Topic::Home => html! {
                <ul class="uk-breadcrumb">
                    <li><a href="#">{ "Docs" }</a></li>
                    <li><span>{ "Get Started" }</span></li>
                </ul>
            },
            Topic::IdentityFundamentals => html! {
                match self.sub_topic {
                    SubTopic::Home => html! {
                        <ul class="uk-breadcrumb">
                            <li><a href="#">{ "Docs" }</a></li>
                            <li><span>{ "Get Started" }</span></li>
                            <li><span>{ "Identity Fundamentals" }</span></li>
                        </ul>
                    },
                    SubTopic::IntroductionToIAM => html! {
                        <ul class="uk-breadcrumb">
                            <li><a href="#">{ "Docs" }</a></li>
                            <li><span>{ "Get Started" }</span></li>
                            <li><span>{ "Identity Fundamentals" }</span></li>
                            <li><span>{ "Introduction to Identity and Access Management (IAM)" }</span></li>
                        </ul>
                    },
                    SubTopic::AuthenticationVsAuthorization => html! {
                        <ul class="uk-breadcrumb">
                            <li><a href="#">{ "Docs" }</a></li>
                            <li><span>{ "Get Started" }</span></li>
                            <li><span>{ "Identity Fundamentals" }</span></li>
                            <li><span>{ "Authentication vs. Authorization" }</span></li>
                        </ul>
                    }
                }
            },
            _ => html! {}
        }
    }
}