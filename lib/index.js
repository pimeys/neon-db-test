const { DatabaseClient } = require('../native')
const bent = require('bent')
const getJSON = bent('json')
const app = require('fastify')({ logger: false })

const port = 3000
const db = new DatabaseClient("file:test.db")

app.get('/users', ({}, res) => db.users((users) => {
    res.send(JSON.stringify(users))
}))

app.get('/users_http', async function({}, res) {
    const response = await getJSON('http://localhost:3030/users/')
    res.send(JSON.stringify(response))
})

app.listen(port, (err, addr) => {
    if (err) throw err
    console.log(`Example app listening on port ${addr}!`)
})
