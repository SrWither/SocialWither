"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var express = require("express");
var WebSocket = require("ws");
var http = require("http");
var url_1 = require("url");
var app = express();
var server = http.createServer(app);
var wss = new WebSocket.Server({ server: server });
var onlineUsers = [];
wss.on('connection', function (ws, req) {
    console.log('New client connected');
    var request = req;
    // Parse the URL to extract query parameters
    var query = (0, url_1.parse)(request.url || '', true).query;
    var username = query === null || query === void 0 ? void 0 : query.username;
    var userIdString = query === null || query === void 0 ? void 0 : query.userId;
    if (!username || !userIdString) {
        console.log('Invalid connection request. Closing connection.');
        ws.close();
        return;
    }
    var userId = parseInt(userIdString, 10);
    if (isNaN(userId)) {
        console.log('Invalid user ID. Closing connection.');
        ws.close();
        return;
    }
    onlineUsers.push({ id: userId, username: username, ws: ws });
    broadcastOnlineUsers();
    ws.on('close', function () {
        console.log("".concat(username, " disconnected"));
        var index = onlineUsers.findIndex(function (user) { return user.ws === ws; });
        if (index !== -1) {
            onlineUsers.splice(index, 1);
            broadcastOnlineUsers();
        }
    });
});
function broadcastOnlineUsers() {
    var users = onlineUsers.map(function (user) { return ({ id: user.id, username: user.username }); });
    broadcast({ type: 'onlineUsers', users: users });
}
function broadcast(message) {
    onlineUsers.forEach(function (user) {
        if (user.ws.readyState === WebSocket.OPEN) {
            user.ws.send(JSON.stringify(message));
        }
    });
}
server.listen(3000, function () {
    console.log('WebSocket server listening on port 3000');
});
