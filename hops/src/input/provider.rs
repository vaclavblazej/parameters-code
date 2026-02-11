use crate::{
    data::{id::PreviewId, link::Link},
    input::{
        build::CollectionBuilder,
        raw::{RawProvider, RawProviderLink},
    },
};

pub struct RawDataProvider {
    provider: RawProvider,
    links: Vec<RawProviderLink>,
    format_url: Box<dyn Fn(&str) -> String>,
}

impl RawDataProvider {
    pub fn new(provider: RawProvider, format_url: Box<dyn Fn(&str) -> String>) -> Self {
        RawDataProvider {
            provider,
            links: Vec::new(),
            format_url,
        }
    }

    pub fn link<T>(mut self, set_id: &PreviewId<T>, link_id: &str) -> Self {
        let provider_id = self.provider.id.preview();
        let provider_link = RawProviderLink {
            provider: provider_id,
            link: Link {
                name: String::new(),
                url: (self.format_url)(link_id),
            },
        };
        self.links.push(provider_link);
        self
    }

    pub fn done(self, builder: &mut CollectionBuilder) {
        let RawDataProvider {
            provider,
            mut links,
            format_url: _,
        } = self;
        builder.data.providers.push(provider);
        builder.data.provider_links.append(&mut links);
    }
}
