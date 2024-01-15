import * as express from 'express';
import * as WebSocket from 'ws';
import * as http from 'http';
import { IncomingMessage } from 'http';
import { parse } from 'url';

const app = express();
const server = http.createServer(app);
const wss = new WebSocket.Server({ server });

interface ExtendedRequest extends IncomingMessage {
  query: { [key: string]: string | undefined };
}

const onlineUsers: { id: number; username: string; ws: WebSocket }[] = [];

wss.on('connection', (ws, req) => {
  console.log('New client connected');

  const request = req as ExtendedRequest;

  // Parse the URL to extract query parameters
  const { query } = parse(request.url || '', true);

  const username = query?.username;
  const userIdString = query?.userId;

  if (!username || !userIdString) {
    console.log('Invalid connection request. Closing connection.');
    ws.close();
    return;
  }

  const userId = parseInt(userIdString as string, 10);

  if (isNaN(userId)) {
    console.log('Invalid user ID. Closing connection.');
    ws.close();
    return;
  }

  onlineUsers.push({ id: userId, username: username as string, ws });
  broadcastOnlineUsers();

  ws.on('close', () => {
    console.log(`${username} disconnected`);
    const index = onlineUsers.findIndex((user) => user.ws === ws);
    if (index !== -1) {
      onlineUsers.splice(index, 1);
      broadcastOnlineUsers();
    }
  });
});

function broadcastOnlineUsers() {
  const users = onlineUsers.map((user) => ({ id: user.id, username: user.username }));
  broadcast({ type: 'onlineUsers', users });
}

function broadcast(message: any) {
  onlineUsers.forEach((user) => {
    if (user.ws.readyState === WebSocket.OPEN) {
      user.ws.send(JSON.stringify(message));
    }
  });
}

server.listen(3000, () => {
  console.log('WebSocket server listening on port 3000');
});
