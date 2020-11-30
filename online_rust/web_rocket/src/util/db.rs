use mysql::prelude::*;
use mysql::*;

static mut DB_POOL: Option<Pool> = None;

pub fn init_db(min: usize, max: usize) {
    let dsn = "mysql://root:root@localhost:3306/dbname";
    unsafe {
        DB_POOL = Some(Pool::new_manual(min, max, dsn).unwrap());
    }
}

pub fn db() -> Result<PooledConn> {
    unsafe {
        match &DB_POOL {
            Some(pool) => pool.get_conn(),
            None => {
                init_db(5,10);
                db()
                //panic!("未初始化 DB ... 请先调用 init_db");
            }
        }
    }
}