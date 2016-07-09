var express = require('express');

var app = express();


app.get('/', function(req, res) {
  res.sendFile(__dirname + '/app/index.html');
});

app.get('/build/common.js', function(req, res) {
  res.sendFile(__dirname + '/build/common.js');
});
app.get('/build/angular2.js', function(req, res) {
  res.sendFile(__dirname + '/build/angular2.js');
});
app.get('/build/app.js', function(req, res) {
  res.sendFile(__dirname + '/build/app.js');
});
app.get('/node_modules/angular2/bundles/angular2-polyfills.js', function(req, res) {
  res.sendFile(__dirname + '/node_modules/angular2/bundles/angular2-polyfills.js');
});

app.listen(8000);