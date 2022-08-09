const status = document.querySelector('#status');
const connectButton = document.querySelector('#connect');
connectButton.disabled = true;
const log = document.querySelector('#log');
const textInput = document.querySelector('#text');

/** @type {WebSocket | null} */
let socket = null;
let websocketUrl = '';

//

function logStatus(msg, type = 'status') {
  log.innerHTML += `<p class="msg msg--${type}">${msg}</p>`;
  log.scrollTop += 1000;
}

function connect() {
  disconnect();
  logStatus('Connecting...');
  socket = new WebSocket(websocketUrl);

  socket.onopen = () => {
    logStatus('Connected');
    updateConnectionStatus();
  };

  socket.onmessage = (ev) => {
    logStatus('Received: ' + ev.data, 'message');
  };

  socket.onclose = () => {
    logStatus('Disconnected');
    socket = null;
    updateConnectionStatus();
  };
}

function disconnect() {
  if (socket) {
    logStatus('Disconnecting...');
    socket.close();
    socket = null;

    updateConnectionStatus();
  }
}

function updateConnectionStatus() {
  if (socket) {
    status.style.backgroundColor = 'transparent';
    status.style.color = 'green';
    status.textContent = `connected`;
    connectButton.innerHTML = 'Disconnect';
    textInput.focus();
  } else {
    status.style.backgroundColor = 'red';
    status.style.color = 'white';
    status.textContent = 'disconnected';
    connectButton.textContent = 'Connect';
  }
}

document.querySelector('#urlform').addEventListener('submit', (event) => {
  event.preventDefault();
  connectButton.disabled = false;
  websocketUrl = document.querySelector('#url').value;
});

connectButton.addEventListener('click', () => {
  if (socket) {
    disconnect();
  } else {
    connect();
  }
  updateConnectionStatus();
});

document.querySelector('#chatform').addEventListener('submit', (ev) => {
  ev.preventDefault();

  const text = textInput.value;

  logStatus('Sending: ' + text);
  socket.send(text);

  textInput.value = '';
  textInput.focus();
});

updateConnectionStatus();

// Extract userId from event.data
