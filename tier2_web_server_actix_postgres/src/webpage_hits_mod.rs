//! webpage_hits_mod.rs

type DbPool = actix_web::web::Data<deadpool_postgres::Pool>;
use crate::actix_mod::get_value_from_query;
use crate::actix_mod::return_response_no_cache;

/// scoped actix routing near the implementation code
/// actix have this magic data extraction thing to accommodate different parameters
pub fn config_route_webpage_hits(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(webpage_hits_list)
        .service(webpage_hits_new)
        .service(webpage_hits_edit)
        .service(webpage_hits_insert)
        .service(webpage_hits_read)
        .service(webpage_hits_update)
        .service(webpage_hits_delete);
}

// list all webpages and counts
#[actix_web::get("/webpage_hits_list")]
pub async fn webpage_hits_list(db_pool: DbPool) -> actix_web::HttpResponse {
    println!("webpage_hits_list");
    // 1. get data from database
    let client = db_pool.get().await.unwrap();
    let row_set = client
        .query("SELECT id, webpage, hit_count from webpage_hits;", &[])
        .await
        .unwrap();
    // 2. presentation with replaceable mustache
    let body = r#"
<html>
<head>
<link rel="stylesheet" href="/database_web_ui_on_server/css/database_web_ui_on_server.css" />
</head>
<body>
<h1>webpage_hits_list</h1>
<div class="table">
<div></div> <div></div> <div>id</div> <div>webpage</div> <div>hit_count</div>
{content}
</div>
<br/>
<div>
<button onclick="location.href='webpage_hits_new'" >New record</button>
</div>
</body></html>
"#;
    // 3. mix presentation and data because of html
    let mut content = String::new();
    for r in row_set {
        let id: i32 = r.get(0);
        let webpage: String = r.get(1);
        let hit_count: i32 = r.get(2);
        let actions = format!(
            r#"<div><a class="button" href='webpage_hits_edit?id={id}'>edit</a></div> <div><a class="button" href='webpage_hits_delete?id={id}'>delete</a></div>"#
        );
        content.push_str(&format!(
            r##"{actions}<div><a href='webpage_hits_read?id={id}'>{id}</a></div><div>{webpage}</div><div>{hit_count}</div>"##
        ));
    }
    let body = body.replace("{content}", &content);
    // 4. return response
    return_response_no_cache(body)
}

// new record UI
#[actix_web::get("/webpage_hits_new")]
pub async fn webpage_hits_new(db_pool: DbPool) -> actix_web::HttpResponse {
    println!("webpage_hits_new");
    // 1. get data from database
    let client = db_pool.get().await.unwrap();
    let row_set = client
        .query(
            "SELECT id, webpage, hit_count from webpage_hits_new();",
            &[],
        )
        .await
        .unwrap();
    // 2. presentation
    // write and test the html+css on https://jsfiddle.net/
    let body = String::from(
        r#"
<html>
<head>
<link rel="stylesheet" href="/database_web_ui_on_server/css/database_web_ui_on_server.css" />
</head>
<body>
<h1>webpage_hits_new</h1>
<form action="webpage_hits_insert" method="get" >
<p>
<label for="id">Id:</label>
<input type="text" id="id" name="id" readonly="readonly" value="{id}" />
</p>
<p>
<label for="id">Webpage:</label>
<input type="text" id="webpage" name="webpage" value="{webpage}" />
</p>
<p>
<label for="id">Count:</label>
<input type="text" id="hit_count" name="hit_count" value="{hit_count}" />
</p>
<input type="submit" style="button" value="Submit">
<button type="button" onclick="location.href='webpage_hits_list'" >Cancel</button>
</form>
</body></html>
"#,
    );
    // 3. mix presentation and data because of html
    if row_set.len() != 1 {
        panic!("row_set is not 1");
    }
    let r = row_set.iter().next().unwrap();
    let id: i32 = r.get(0);
    let webpage: String = r.get(1);
    let hit_count: i32 = r.get(2);
    let body = body
        .replace("{id}", &id.to_string())
        .replace("{webpage}", &webpage)
        .replace("{hit_count}", &hit_count.to_string());
    // 4. return response
    return_response_no_cache(body)
}
// edit record UI
#[actix_web::get("/webpage_hits_edit")]
pub async fn webpage_hits_edit(
    db_pool: DbPool,
    query: actix_web::web::Query<Vec<(String, String)>>,
) -> actix_web::HttpResponse {
    println!("webpage_hits_edit");
    // 1. send parameters to database
    let client = db_pool.get().await.unwrap();
    let id: i32 = get_value_from_query(&query, "id").unwrap().parse().unwrap();
    let row_set = client
        .query(
            "SELECT id, webpage, hit_count from webpage_hits_read($1);",
            &[&id],
        )
        .await
        .unwrap();
    // 2. presentation
    // write and test the html+css on https://jsfiddle.net/
    let body = String::from(
        r#"
    <html>
<head>
<link rel="stylesheet" href="/database_web_ui_on_server/css/database_web_ui_on_server.css" />
</head>
<body>
    <h1>webpage_his_edit</h1>
    <form action="webpage_hits_update" method="get" >
    <p>
    <label for="id">Id:</label>
    <input type="text" id="id" name="id" readonly="readonly" value="{id}" />
    </p>
    <p>
    <label for="id">Webpage:</label>
    <input type="text" id="webpage" name="webpage" value="{webpage}" />
    </p>
    <p>
    <label for="id">Hit_count:</label>
    <input type="text" id="hit_count" name="hit_count" value="{hit_count}" />
    </p>
    <button type="submit" style="button" value="Submit">Submit</button>
    <button type="button" style="button" onclick="location.href='webpage_hits_list'" >Cancel</button>
    </form>
    </body></html>
    "#,
    );
    // 3. mix presentation and data because of html
    if row_set.len() != 1 {
        panic!("row_set is not 1");
    }
    let r = row_set.iter().next().unwrap();
    let id: i32 = r.get(0);
    let webpage: String = r.get(1);
    let hit_count: i32 = r.get(2);
    let body = body
        .replace("{id}", &id.to_string())
        .replace("{webpage}", &webpage)
        .replace("{hit_count}", &hit_count.to_string());
    // 4. return response
    return_response_no_cache(body)
}
// CRUD - create(insert)
#[actix_web::get("/webpage_hits_insert")]
pub async fn webpage_hits_insert(
    db_pool: DbPool,
    query: actix_web::web::Query<Vec<(String, String)>>,
) -> actix_web::HttpResponse {
    println!("webpage_hits_insert");
    // 1. send parameters to database
    // the number of parameters and types must be exact
    // and get data from database
    let client = db_pool.get().await.unwrap();
    let id: i32 = get_value_from_query(&query, "id").unwrap().parse().unwrap();
    let webpage: String = get_value_from_query(&query, "webpage").unwrap().to_string();
    let hit_count: i32 = get_value_from_query(&query, "hit_count")
        .unwrap()
        .parse()
        .unwrap();
    let row_set = client
        .query(
            "SELECT id, webpage, hit_count from webpage_hits_insert($1, $2, $3);",
            &[&id, &webpage, &hit_count],
        )
        .await
        .unwrap();
    // 2. presentation
    // write and test the html+css on https://jsfiddle.net/
    let body = String::from(
        r#"
    <html>
<head>
<link rel="stylesheet" href="/database_web_ui_on_server/css/database_web_ui_on_server.css" />
</head>
<body>
    <h1>webpage_hits_insert</h1>
    <form>
    <p>
    <label for="id">Id:</label>
    <input type="text" id="id" name="id" readonly="readonly" value="{id}" />
    </p>
    <p>
    <label for="id">Webpage:</label>
    <input type="text" id="webpage" name="webpage" readonly="readonly" value="{webpage}" />
    </p>
    <p>
    <label for="id">Count:</label>
    <input type="text" id="hit_count" name="hit_count" readonly="readonly" value="{hit_count}" />
    </p>
    </form>
<div>
<button onclick="location.href='webpage_hits_list'" >List</button>
</div>
    </body></html>
    "#,
    );
    // 3. mix presentation and data because of html
    if row_set.len() != 1 {
        panic!("row_set is not 1");
    }
    let r = row_set.iter().next().unwrap();
    let id: i32 = r.get(0);
    let webpage: String = r.get(1);
    let hit_count: i32 = r.get(2);
    let body = body
        .replace("{id}", &id.to_string())
        .replace("{webpage}", &webpage)
        .replace("{hit_count}", &hit_count.to_string());
    // 4. return response
    return_response_no_cache(body)
}
// CRUD - read one record
#[actix_web::get("/webpage_hits_read")]
pub async fn webpage_hits_read(
    db_pool: DbPool,
    query: actix_web::web::Query<Vec<(String, String)>>,
) -> actix_web::HttpResponse {
    println!("webpage_hits_read");
    // 1. send parameters to database
    let client = db_pool.get().await.unwrap();
    let id: i32 = get_value_from_query(&query, "id").unwrap().parse().unwrap();
    let row_set = client
        .query(
            "SELECT id, webpage, hit_count from webpage_hits_read($1);",
            &[&id],
        )
        .await
        .unwrap();
    // 2. presentation
    // write and test the html+css on https://jsfiddle.net/
    let body = String::from(
        r#"
    <html>
<head>
<link rel="stylesheet" href="/database_web_ui_on_server/css/database_web_ui_on_server.css" />
</head>
<body>
    <h1>webpage_hits_read</h1>
    <form >
    <p>
    <label for="id">Id:</label>
    <input type="text" id="id" name="id" readonly="readonly" value="{id}" />
    </p>
    <p>
    <label for="id">Webpage:</label>
    <input type="text" id="webpage" name="webpage" readonly="readonly" value="{webpage}" />
    </p>
    <p>
    <label for="id">Count:</label>
    <input type="text" id="hit_count" name="hit_count" readonly="readonly" value="{hit_count}" />
    </p>
    </form>
<div>
<button onclick="location.href='webpage_hits_list'" >List</button>
</div>
    </body></html>
    "#,
    );
    // 3. mix presentation and data because of html
    if row_set.len() != 1 {
        panic!("row_set is not 1");
    }
    let r = row_set.iter().next().unwrap();
    let id: i32 = r.get(0);
    let webpage: String = r.get(1);
    let hit_count: i32 = r.get(2);
    let body = body
        .replace("{id}", &id.to_string())
        .replace("{webpage}", &webpage)
        .replace("{hit_count}", &hit_count.to_string());
    // 4. return response
    return_response_no_cache(body)
}
// CRUD - update
#[actix_web::get("/webpage_hits_update")]
pub async fn webpage_hits_update(
    db_pool: DbPool,
    query: actix_web::web::Query<Vec<(String, String)>>,
) -> actix_web::HttpResponse {
    println!("webpage_hits_update");
    // 1. send parameters to database
    // the number of parameters and types must be exact
    // and get data from database
    let client = db_pool.get().await.unwrap();
    let id: i32 = get_value_from_query(&query, "id").unwrap().parse().unwrap();
    let webpage: String = get_value_from_query(&query, "webpage").unwrap().to_string();
    let hit_count: i32 = get_value_from_query(&query, "hit_count")
        .unwrap()
        .parse()
        .unwrap();
    let row_set = client
        .query(
            "SELECT id, webpage, hit_count from webpage_hits_update($1, $2, $3);",
            &[&id, &webpage, &hit_count],
        )
        .await
        .unwrap();
    // 2. presentation
    // write and test the html+css on https://jsfiddle.net/
    let body = String::from(
        r#"
    <html>
<head>
<link rel="stylesheet" href="/database_web_ui_on_server/css/database_web_ui_on_server.css" />
</head>
<body>
    <h1>webpage_hits_update</h1>
    <form >
    <p>
    <label for="id">Id:</label>
    <input type="text" id="id" name="id" readonly="readonly" value="{id}" />
    </p>
    <p>
    <label for="id">Webpage:</label>
    <input type="text" id="webpage" name="webpage" readonly="readonly" value="{webpage}" />
    </p>
    <p>
    <label for="id">Count:</label>
    <input type="text" id="hit_count" name="hit_count" readonly="readonly" value="{hit_count}" />
    </p>
    </form>
<div>
<button onclick="location.href='webpage_hits_list'" >List</button>
</div>
    </body></html>
    "#,
    );
    // 3. mix presentation and data because of html
    if row_set.len() != 1 {
        panic!("row_set is not 1");
    }
    let r = row_set.iter().next().unwrap();
    let id: i32 = r.get(0);
    let webpage: String = r.get(1);
    let hit_count: i32 = r.get(2);
    let body = body
        .replace("{id}", &id.to_string())
        .replace("{webpage}", &webpage)
        .replace("{hit_count}", &hit_count.to_string());
    // 4. return response
    return_response_no_cache(body)
}
// CRUD - delete
#[actix_web::get("/webpage_hits_delete")]
pub async fn webpage_hits_delete(
    db_pool: DbPool,
    query: actix_web::web::Query<Vec<(String, String)>>,
) -> actix_web::HttpResponse {
    println!("webpage_hits_delete");
    // 1. send parameters to database
    // the number of parameters and types must be exact
    // and get data from database
    let client = db_pool.get().await.unwrap();
    let id: i32 = get_value_from_query(&query, "id").unwrap().parse().unwrap();
    client
        .query("SELECT webpage_hits_delete($1);", &[&id])
        .await
        .unwrap();
    // 2. presentation
    // write and test the html+css on https://jsfiddle.net/
    let body = String::from(
        r#"
<html>
<head>
<link rel="stylesheet" href="/database_web_ui_on_server/css/database_web_ui_on_server.css" />
</head>
<body>
<h1>webpage_hits_delete</h1>
<p>Record deleted!</p>
<div>
<button onclick="location.href='webpage_hits_list'" >Return to list</button>
</div>
</body></html>
"#,
    );
    // 3. mix presentation and data because of html
    // 4. return response
    return_response_no_cache(body)
}
