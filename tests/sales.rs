use fake::{Fake, Faker};

use utils::spawn_app;

mod dummies;
mod utils;

#[tokio::test]
async fn sale() {
    let app = spawn_app().await;
    let expected_sale: dummies::Sale = Faker.fake();
    sqlx::query!(
        r#"
        INSERT INTO sales (id, prev_owner, curr_owner, token_id, price, date)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        expected_sale.id,
        expected_sale.prev_owner,
        expected_sale.curr_owner,
        expected_sale.token_id,
        expected_sale.price,
        expected_sale.date
    )
    .execute(&app.db_pool)
    .await
    .expect("Failed to execute query");

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/sales", app.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    let actual_sales = response.json::<Vec<dummies::Sale>>().await.unwrap();
    assert_eq!(actual_sales.len(), 1);
    assert_eq!(actual_sales[0].id, expected_sale.id);
}

#[tokio::test]
async fn sales_valid_when_in_proper_type_or_omitted_and_replaced_to_default_values_by_validators() {
    let app = spawn_app().await;
    let sales = fake::vec![dummies::Sale; 200];
    for sale in sales {
        sqlx::query!(
            r#"
            INSERT INTO sales (id, prev_owner, curr_owner, token_id, price, date)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            sale.id,
            sale.prev_owner,
            sale.curr_owner,
            sale.token_id,
            sale.price,
            sale.date
        )
        .execute(&app.db_pool)
        .await
        .expect("Failed to execute query");
    }

    let client = reqwest::Client::new();
    let queries_and_expectations = [
        ("/sales", 100),
        ("/sales?offset=2", 100),
        ("/sales?offset=199", 1),
        ("/sales?offset=200", 0),
        ("/sales?offset=150", 50),
        ("/sales?limit=50", 50),
        ("/sales?limit=250", 200),
        ("/sales?limit=0", 0),
        ("/sales?offset=0&limit=0", 0),
        ("/sales?offset=10&limit=25", 25),
        ("/sales?offset=200&limit=10", 0),
        ("/sales?offset=199&limit=11", 1),
    ];

    for (idx, (query, expectation)) in queries_and_expectations.iter().enumerate() {
        let url = format!("{}{}", app.address, query);
        let response = client
            .get(&url)
            .send()
            .await
            .unwrap_or_else(|_| panic!("Failed to execute request: {url}"));
        assert_eq!(response.status().as_u16(), 200);

        let actual_sales = response.json::<Vec<dummies::Sale>>().await.unwrap();
        assert_eq!(
            actual_sales.len(),
            *expectation,
            "length of sales not the same. query is: {} with index: {}",
            query,
            idx
        );
    }
}
