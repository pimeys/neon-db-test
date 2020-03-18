const { DatabaseClient } = require('../native')
const express = require('express')

async function main() {
    const app = express()
    const port = 3000
    const db = new DatabaseClient("file:test.db")

    app.get('/users', (req, res) => db.users((users) => {
        res.send(JSON.stringify(users))
    }))

    const server = app.listen(port, () => console.log(`Example app listening on port ${port}!`))

    await new Promise(resolve => server.on('close', resolve))
}

main().catch(console.error).then(() => {
    console.log('exiting')
    process.exit(0)
})
