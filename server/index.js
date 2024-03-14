const app = require('express')();
const http = require('http').Server(app);
const cors = require('cors');""
const io = require('socket.io')(http, {
    allowUpgrades: false,
    // transports: ["polling"],
    // transports: ["websocket", "polling"], // use WebSocket first, if available
    cors: { origin: "*" },
});

io.on('connection', (socket) => {
    socket.on('test', function (msg) {
        console.log("test: " + msg);
    });

    console.log("connected");
});

setInterval(() => {
    io.emit("send-client-message", "message");
}, 1000);

http.listen(3000, () => {
    console.log('listening on *:3000');
});