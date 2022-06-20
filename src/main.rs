use gluesql::prelude::*;

fn main() {
    let storage = SledStorage::new("data/doc-db").unwrap();
    let mut glue = Glue::new(storage);
    let sqls = vec![
        "DROP TABLE IF EXISTS Glue;",
        "CREATE TABLE Glue (id INTEGER, name TEXT);",
        "INSERT INTO Glue VALUES (100, 'kim');",
        "INSERT INTO Glue VALUES (200, 'jang');",
		"INSERT INTO Glue VALUES (300, 'lee');",
		"INSERT INTO Glue VALUES (400, 'park');",
        "SELECT * FROM Glue WHERE id > 100 AND name != 'jang';",
    ];

    for sql in sqls {
        let output = glue.execute(sql).unwrap();
        println!("{:?}", output)
    }
}
