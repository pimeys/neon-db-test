const { DatabaseClient } = require('../native');

const db = new DatabaseClient("file:test.db");
db.users((res) => {
    console.log(res);
});
