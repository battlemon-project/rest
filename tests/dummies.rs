use fake::faker::address::en::Geohash;
use fake::faker::chrono::raw::DateTime;
use fake::faker::lorem::en::Word;
use fake::faker::number::raw::NumberWithFormat;
use fake::locales::EN;
use fake::uuid::UUIDv4;
use fake::{Dummy, Fake, Faker};
use nft_models::{Lemon, ModelKind};
use rand::Rng;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Sale {
    pub id: uuid::Uuid,
    pub prev_owner: String,
    pub curr_owner: String,
    pub token_id: String,
    pub price: rust_decimal::Decimal,
    pub date: chrono::DateTime<chrono::Utc>,
}

impl Dummy<Faker> for Sale {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let scale = rng.gen_range(0..=24);
        let lo = rng.gen();
        let mid = rng.gen();
        let price = rust_decimal::Decimal::from_parts(lo, mid, 0, false, scale);
        Self {
            id: UUIDv4.fake::<uuid::Uuid>(),
            prev_owner: format!("{}.near", Word().fake::<String>()),
            curr_owner: format!("{}.near", Word().fake::<String>()),
            token_id: NumberWithFormat(EN, "^########").fake::<String>(),
            price,
            date: DateTime(EN).fake(),
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct NftToken {
    pub id: uuid::Uuid,
    pub owner_id: String,
    pub token_id: String,
    pub media: String,
    pub model: ModelKind,
    pub db_created_at: chrono::DateTime<chrono::Utc>,
}

impl Dummy<Faker> for NftToken {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let probability = fake::vec![u8; 4]
            .try_into()
            .expect("Couldn't convert to array with length 4");
        let model = ModelKind::Lemon(Lemon::from_random(&probability));
        let token_id = rng.gen::<u64>().to_string();
        
        Self {
            id: UUIDv4.fake::<uuid::Uuid>(),
            owner_id: format!("{}.near", Word().fake::<String>()),
            token_id,
            media: Geohash(24).fake(),
            model,
            db_created_at: DateTime(EN).fake(),
        }
    }
}
