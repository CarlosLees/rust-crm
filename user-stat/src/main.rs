use std::{collections::HashSet, hash::Hash, time::Instant};

use anyhow::Result;
use chrono::{DateTime, Days, Utc};
use fake::{
    faker::{chrono::zh_cn::DateTimeBetween, internet::en::SafeEmail, name::zh_cn::Name},
    Dummy, Fake, Faker,
};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use sqlx::{Executor, PgPool};

#[derive(Debug, Serialize, Deserialize, Dummy, Clone, PartialEq, Eq)]
enum Gender {
    Female,
    Male,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Dummy, Clone, PartialEq, Eq)]
pub struct UserStat {
    #[dummy(faker = "UniqueEmail")]
    email: String,

    #[dummy(faker = "Name()")]
    name: String,

    gender: Gender,

    #[dummy(faker = "DateTimeBetween(start(365*5), end())")]
    created_at: DateTime<Utc>,

    #[dummy(faker = "DateTimeBetween(start(30), end())")]
    last_visited_at: DateTime<Utc>,

    #[dummy(faker = "DateTimeBetween(start(90), end())")]
    last_watch_at: DateTime<Utc>,

    #[dummy(faker = "IntList(50, 100000, 100000)")]
    recent_watched: Vec<i32>,

    #[dummy(faker = "IntList(50, 200000, 100000)")]
    viewed_but_not_started: Vec<i32>,

    #[dummy(faker = "IntList(50, 300000, 100000)")]
    started_but_not_finished: Vec<i32>,

    #[dummy(faker = "IntList(50, 400000, 100000)")]
    finished: Vec<i32>,

    #[dummy(faker = "DateTimeBetween(start(45), end())")]
    last_email_notification: DateTime<Utc>,

    #[dummy(faker = "DateTimeBetween(start(15), end())")]
    last_in_app_notification: DateTime<Utc>,

    #[dummy(faker = "DateTimeBetween(start(90), end())")]
    last_sms_notificaiton: DateTime<Utc>,
}

impl Hash for UserStat {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.email.hash(state);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let pool = PgPool::connect("postgres://postgres:lsw0516@127.0.0.1/stats").await?;
    for i in 1..500 {
        let users: HashSet<_> = (0..10000)
            .into_iter()
            .map(|_| Faker.fake::<UserStat>())
            .collect();

        let start = Instant::now();
        println!("Insert Batch {}...", i);
        bulk_insert(users, &pool).await?;
        println!("Batch {} inserted in {:?}", i, start.elapsed());
    }
    Ok(())
}

async fn bulk_insert(users: HashSet<UserStat>, pool: &PgPool) -> Result<()> {
    let mut tx = pool.begin().await?;
    for user in users {
        let query = sqlx::query(
            r#"
            insert into user_stats(email,name,created_at,last_visited_at,last_watch_at,recent_watched,viewed_but_not_started,
            started_but_not_finished,finished,last_email_notification,last_in_app_notification,last_sms_notificaiton) 
            values($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12)
            "#
        )
        .bind(&user.email)
        .bind(&user.name)
        .bind(&user.created_at)
        .bind(&user.last_visited_at)
        .bind(&user.last_watch_at)
        .bind(&user.recent_watched)
        .bind(&user.viewed_but_not_started)
        .bind(&user.started_but_not_finished)
        .bind(&user.finished)
        .bind(&user.last_email_notification)
        .bind(&user.last_in_app_notification)
        .bind(&user.last_sms_notificaiton);

        tx.execute(query).await?;
    }
    tx.commit().await?;
    Ok(())
}

fn start(days: u64) -> DateTime<Utc> {
    DateTime::from(Utc::now())
        .checked_sub_days(Days::new(days))
        .unwrap()
}

fn end() -> DateTime<Utc> {
    DateTime::from(Utc::now())
}

struct IntList(pub i32, pub i32, pub i32);

impl Dummy<IntList> for Vec<i32> {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &IntList, rng: &mut R) -> Self {
        let (max, start, len) = (config.0, config.1, config.2);
        let size = rng.gen_range(0..max);
        (0..size)
            .map(|_| rng.gen_range(start..start + len))
            .collect()
    }
}

struct UniqueEmail;

const ALPHABET: [char; 36] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

impl Dummy<UniqueEmail> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &UniqueEmail, rng: &mut R) -> Self {
        let email: String = SafeEmail().fake_with_rng(rng);
        let id = nanoid!(8, &ALPHABET);

        let at = email.find('@').unwrap();
        format!("{}.{}{}", &email[..at], id, &email[at..])
    }
}
