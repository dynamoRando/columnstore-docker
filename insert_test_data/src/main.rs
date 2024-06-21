use core::num;

use anyhow::Error;
use chrono::{Duration, NaiveDate};
use mysql::prelude::*;
use mysql::*;
use rand::prelude::*;

#[derive(Debug)]
struct Post {
    pub id: u32,
    pub poster_id: u32,
    pub post_ts: String,
}

fn main() -> Result<()> {
    let url = "mysql://database:password@localhost:3306/test";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;

    // for verification purposes
    // let num_posts = 3;

    // note that you can go higher here, but it will take MariaDB CS time to catch up
    let num_posts = 1_500_000;

    for post_id in 1..num_posts {
        let post = get_post(post_id);

        println!("{post:?}");

        let sql = r#"
        INSERT INTO
            posts (post_id, poster_id, post_ts)
        VALUES
            (:post_id, :poster_id, :post_ts);"#;

        conn.exec_batch(
            sql,
            vec![params! {
                "post_id" => post.id,
                "poster_id" => post.poster_id,
                "post_ts" => post.post_ts,
            }],
        )
        .unwrap();
    }

    Ok(())
}

fn get_post(id: u32) -> Post {
    let mut rng = thread_rng();
    let poster_id: u32 = rng.gen_range(1..4);

    let start_date = NaiveDate::parse_from_str("1990-01-01", "%Y-%m-%d").unwrap();
    let end_date = NaiveDate::parse_from_str("2024-01-01", "%Y-%m-%d").unwrap();
    let post_ts = random_date_in_range(&mut rng, start_date, end_date);

    Post {
        id,
        poster_id,
        post_ts: post_ts.to_string(),
    }
}

pub fn random_date_in_range(
    rng: &mut rand::rngs::ThreadRng,
    start: NaiveDate,
    end: NaiveDate,
) -> NaiveDate {
    let days_in_range = (end - start).num_days();
    let random_days: i64 = rng.gen_range(0..days_in_range);
    start + Duration::days(random_days)
}
