use rusqlite::{Connection, Result, params};
use std::{collections::HashMap, println, vec};

fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "create table if not exists cat_colors (
            id integer primary key,
            name text not null unique
        )",
        (),
    )?;
    conn.execute(
        "create table if not exists cats (
        id integer primary key,
        name text not null,
        color_id integer not null references cat_colors(id)
    )",
        (),
    )?;
    Ok(())
}

#[derive(Debug)]
struct Cat {
    name: String,
    color: String,
}

#[warn(unused)]
fn insert_cats(conn: &Connection, cat_colors: &HashMap<String, Vec<&str>>) -> Result<()> {
    for (color, catnames) in cat_colors {
        conn.execute("INSERT INTO cat_colors (name) VALUES (?1)", [color])?;
        let last_id = conn.last_insert_rowid();

        for cat in catnames {
            conn.execute(
                "INSERT INTO cats (name, color_id) values (?1, ?2)",
                params![cat, last_id],
            )?;
        }
    }

    Ok(())
}

fn select_cats(conn: &Connection) -> Result<Vec<Cat>> {
    let mut stmt = conn.prepare(
        "SELECT c.name, cc.name FROM cats c
            INNER JOIN cat_colors cc
            ON cc.id = c.color_id;",
    )?;

    let cats = stmt.query_map([], |row| {
        Ok(Cat {
            name: row.get(0)?,
            color: row.get(1)?,
        })
    })?;

    cats.collect()
}

fn successful_tx(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;

    tx.execute("delete from cat_colors", [])?;
    tx.execute("insert into cat_colors (name) values (?1)", ["lavender"])?;
    tx.execute("insert into cat_colors (name) values (?1)", ["blue"])?;

    tx.commit()
}

fn rolled_back_tx(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;

    tx.execute("delete from cat_colors", [])?;
    tx.execute("insert into cat_colors (name) values (?1)", ["lavender"])?;
    tx.execute("insert into cat_colors (name) values (?1)", ["blue"])?;
    tx.execute("insert into cat_colors (name) values (?1)", ["lavender"])?;

    tx.commit()
}

fn main() -> Result<()> {
    let mut conn = Connection::open("cats.db")?;
    create_tables(&conn)?;

    let mut cat_colors = HashMap::new();
    cat_colors.insert(String::from("Blue"), vec!["Tigger", "Sammy"]);
    cat_colors.insert(String::from("Black"), vec!["Oreo", "Biscuit"]);

    // insert_cats(&conn, &cat_colors)?;

    successful_tx(&mut conn)?;
    let res = rolled_back_tx(&mut conn);
    assert!(res.is_err());

    for found_cat in select_cats(&conn)? {
        println!(
            "Found cat {:?} {} is {}",
            found_cat, found_cat.name, found_cat.color,
        );
    }

    Ok(())
}
