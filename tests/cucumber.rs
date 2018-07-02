#![feature(fnbox)]
#[macro_use] extern crate cucumber_rust;
extern crate gurka;
extern crate rocket;
extern crate diesel;

fn reset() {
    std::process::Command::new("diesel")
            .args(&["migration", "redo"])
            .output()
            .expect("success");
}

pub struct MyWorld {
    pool: gurka::PgPool,
    client: rocket::local::Client,
    mutator: gurka::DatabaseMutator,
    query: gurka::DatabaseQuery,
    token: Option<String>,
    current_user: Option<gurka::models::User>,
    current_project: Option<gurka::models::Project>
}

impl MyWorld {
    pub fn client(&self) -> &rocket::local::Client {
        &self.client
    }

    pub fn pool(&self) -> &gurka::PgPool {
        &self.pool
    }

    pub fn token(&self) -> Option<&str> {
        match &self.token {
            Some(v) => Some(&v),
            None => None
        }
    }

    pub fn current_user(&self) -> Option<&gurka::models::User> {
        match &self.current_user {
            Some(v) => Some(&v),
            None => None
        }
    }

    pub fn current_project(&self) -> Option<&gurka::models::Project> {
        match &self.current_project {
            Some(v) => Some(&v),
            None => None
        }
    }
}

impl cucumber_rust::World for MyWorld {
}

impl std::default::Default for MyWorld {
    fn default() -> MyWorld {
        reset();
        let server = gurka::make_server();
        let pool = gurka::establish_pool();

        MyWorld {
            client: rocket::local::Client::new(server)
                .expect("rocket to the moon"),
            pool: pool.clone(),
            mutator: gurka::DatabaseMutator::new(pool.clone()),
            query: gurka::DatabaseQuery::new(pool.clone()),
            token: None,
            current_user: None,
            current_project: None
        }
    }
}

mod steps;
use steps::*;

cucumber! {
    features: "./features";
    world: MyWorld;
    steps: &[
        users::steps,
        projects::steps
    ];
    before: || {
        reset();
    }
}
