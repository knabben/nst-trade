
const CREATE_AGENT = 'http://localhost:8086/api/agent';
const LOGIN_AGENT = 'http://localhost:8086/api/authentication';

export function serviceCreateUser(username, password) {
  let data = {"username": username, "password": password};
  return fetch(CREATE_AGENT, {
    method: 'POST',
    body: JSON.stringify(data),
    headers:{'Content-Type': 'application/json'}
  })
}

export function serviceLoginUser(username, password) {
  let data = {"username": username, "password": password};
  return fetch(LOGIN_AGENT, {
    method: 'POST',
    body: JSON.stringify(data),
    headers:{'Content-Type': 'application/json'}
  })
}