//! postgres_mod.rs

pub async fn db_hit_counter_list(
    db_pool: actix_web::web::Data<deadpool_postgres::Pool>,
) -> Result<Vec<tokio_postgres::Row>, tokio_postgres::Error> {
    let client: deadpool_postgres::Client = db_pool.get().await.unwrap();
    // Now we can execute a simple statement that just returns its parameter.
    let rows = client
        .query(
            "
SELECT W.webpage, H.count 
from webpage W 
join hit_counter H on H.webpage_id=W.id
order by W.webpage;",
            &[],
        )
        .await?;
    //return
    Ok(rows)
}
