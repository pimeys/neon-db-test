const { DatabaseClient } = require('../native');

const db = new DatabaseClient("file:test.db");
db.select((res) => {
    console.log(res);
});
