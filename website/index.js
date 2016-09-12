var express = require('express');

var app = express();


app.get('/', function(req, res) {
  res.sendFile(__dirname + '/app/index.html');
});

app.get('/submitproposal', function(req, res) {
  res.sendFile(__dirname + '/app/submitprop.html');
});
app.get('/viewproposal/:proposalName', function(req, res) {
  res.sendFile(__dirname + '/app/viewprop.html');
});
app.get('/voteproposal/:proposalName', function(req, res) {
  res.sendFile(__dirname + '/app/voteprop.html');
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
app.get('/build/submitprop.js', function(req, res) {
  res.sendFile(__dirname + '/build/submitprop.js');
});
app.get('/build/viewprop.js', function(req, res) {
  res.sendFile(__dirname + '/build/viewprop.js');
});
app.get('/build/voteprop.js', function(req, res) {
  res.sendFile(__dirname + '/build/voteprop.js');
});
app.get('/node_modules/angular2/bundles/angular2-polyfills.js', function(req, res) {
  res.sendFile(__dirname + '/node_modules/angular2/bundles/angular2-polyfills.js');
});

app.get('/css/submitprop/css/bootstrap.css', function(req, res) {
  res.sendFile(__dirname + '/app/css/submitprop/css/bootstrap.css');
  console.log(__dirname);
});
app.get('/css/submitprop/css/style.css', function(req, res) {
  res.sendFile(__dirname + '/app/css/submitprop/css/style.css');
});
app.get('/css/submitprop/img/1.jpg', function(req, res) {
  res.sendFile(__dirname + '/app/css/submitprop/img/1.jpg');
});
app.get('/img/safex-logo.png', function(req, res) {
  res.sendFile(__dirname + '/app/css/submitprop/img/safex-logo.png');
});

app.get('/viewproposal/css/viewprop/css/bootstrap.css', function(req, res) {
  res.sendFile(__dirname + '/app/css/viewprop/css/bootstrap.css');
  console.log(__dirname);
});
app.get('/viewproposal/css/viewprop/css/style.css', function(req, res) {
  res.sendFile(__dirname + '/app/css/viewprop/css/style.css');
});


app.get('/viewproposal/css/viewprop/img/1.jpg', function(req, res) {
  res.sendFile(__dirname + '/app/css/viewprop/img/1.jpg');
});

app.get('/viewproposal/img/safex-logo.png', function(req, res) {
  res.sendFile(__dirname + '/app/css/viewprop/img/safex-logo.png');
});

app.get('/css/home/css/bootstrap.css', function(req, res) {
  res.sendFile(__dirname + '/app/css/home/css/bootstrap.css');
  console.log(__dirname);
});
app.get('/css/home/css/style.css', function(req, res) {
  res.sendFile(__dirname + '/app/css/home/css/style.css');
});

app.get('/css/home/img/1.jpg', function(req, res) {
  res.sendFile(__dirname + '/app/css/home/img/1.jpg');
});

app.get('/css/home/img/safex-logo.png', function(req, res) {
  res.sendFile(__dirname + '/app/css/home/img/safex-logo.png');
});

app.get('/voteproposal/css/voteprop/css/bootstrap.css', function(req, res) {
  res.sendFile(__dirname + '/app/css/voteprop/css/bootstrap.css');
  console.log(__dirname);
});
app.get('/voteproposal/css/voteprop/css/style.css', function(req, res) {
  res.sendFile(__dirname + '/app/css/voteprop/css/style.css');
});

app.get('/voteproposal/css/voteprop/img/1.jpg', function(req, res) {
  res.sendFile(__dirname + '/app/css/voteprop/img/1.jpg');
});

app.get('/voteproposal/img/safex-logo.png', function(req, res) {
  res.sendFile(__dirname + '/app/css/voteprop/img/safex-logo.png');
});
app.listen(8000);