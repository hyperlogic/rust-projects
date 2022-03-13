// node client


function between(min, max) {
  return Math.floor(
    Math.random() * (max - min) + min
  )
}

var SERVER_PORT = 34254;
var CLIENT_PORT = between(SERVER_PORT + 1, 50000);
var HOST = '127.0.0.1';

var dgram = require('dgram');
var message = new Buffer('my name is ' + CLIENT_PORT);

var client = dgram.createSocket('udp4');
client.bind(CLIENT_PORT, HOST);

client.on('listening', function() {
  var address = client.address();
  console.log('UDP Server listening on ' + address.address + ':' + address.port);
});

client.on('message', function(message, remote) {
  console.log(remote.address + ':' + remote.port +' - ' + message);
});

client.on('error', function() {
  console.log('error of some kind');
});

client.send(message, 0, message.length, SERVER_PORT, HOST, function(err, bytes) {
  if (err) throw err;
  console.log('UDP message sent to ' + HOST +':'+ SERVER_PORT);
});
client.send(message, 0, message.length, SERVER_PORT, HOST, function(err, bytes) {
  if (err) throw err;
  console.log('UDP message sent to ' + HOST +':'+ SERVER_PORT);
});
client.send(message, 0, message.length, SERVER_PORT, HOST, function(err, bytes) {
  if (err) throw err;
  console.log('UDP message sent to ' + HOST +':'+ SERVER_PORT);
});

//client.close();



