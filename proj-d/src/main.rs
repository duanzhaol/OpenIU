

use serde::{Deserialize,Serialize};
use axum::extract::Form;
use rand::Rng;
//use rand::distributions::{Distribution, Uniform, Normal};
use std::collections::HashMap;
use axum::{http::StatusCode, routing::{get_service,get,post}, Router,response::Html};
use tower_http::{services::ServeDir, trace::TraceLayer};

#[derive(Debug,Deserialize,Serialize)]
struct Query {
    class1: Option<String>,
    class2: Option<String>,
    class3: Option<String>,
    rand_type: Option<i32>,
}


fn add_string_to_table(s:String,table:&mut HashMap<i32,String>)-> () {
    let iter:Vec<&str> = s.split_whitespace().collect();

    for (i,s) in iter.iter().enumerate(){
        let task_name=s.to_string();
        table.insert(i as i32,task_name);
    }
}
async fn random_d(Form(model): Form<Query>) -> String {
    // let s=format!(
    //     "a:{},b:{},c:{},d:{}",
    //     model.class1.unwrap(),
    //     model.class2.unwrap(), 
    //     model.class3.unwrap(),
    //     model.rand_type.unwrap()
    // );
    // println!("{}",s);
    // s
    
    let rand_type=model.rand_type.unwrap();
    let mut task_table = HashMap::new();
    
    match rand_type{
        1 => add_string_to_table(model.class1.unwrap(),&mut task_table),
        2 => add_string_to_table(model.class1.unwrap()+" "+&model.class2.unwrap()[..],&mut task_table),
        3 => add_string_to_table(model.class1.unwrap()+" "+&model.class2.unwrap()[..]+" "+&model.class3.unwrap()[..],&mut task_table),
        _ => panic!("unknown rand type"),

    }
    let max_index=task_table.len();
    let mut rng = rand::thread_rng();
    let die = rng.gen_range(0,max_index) as i32;
    for _ in [..die]{
        rng.gen_range(0,max_index);
    }
    let die = rng.gen_range(0,max_index) as i32;
    task_table.get(&die).unwrap().to_string()
}

async fn index_html() -> axum::response::Html<&'static str> {
    include_str!("ProjD.html").into()
}
#[tokio::main]
async fn main() {
    // our router
    let app = Router::new()
        .route("/D", post(random_d))
        .route("/index", get(index_html));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:23000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}