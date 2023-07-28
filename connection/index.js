const express = require('express')
const cors = require('cors');
const app = new express()
const fs = require('fs')
const { Client } = require('pg')
const client = new Client({
  user: 'postgres',
  host: 'localhost',
  database: 'demo',
  password: '1234567',
  port: 5432,
})

app.use(cors());
client.connect(async function(err) {
  if (err) throw err;
  console.log("Connected!");
  console.log(await getrecord())
});

app.get('/',async (req,res)=>{

    res.json(await getrecord())

})

async function getrecord(){
 let res = await client.query("select * from sniffer")
 return res
}

app.listen(8086,console.log('runnigg.....'))
