import axios from 'axios'

const baseUrl = 'http://localhost:8080'

const request = async function (
  url: string,
  data: object,
  method: string,
  token: string,
): Promise<[object | undefined, string | undefined]> {
  method = method || 'GET'

  const packet = {
    method,
    url: `${baseUrl}${url}`,
    headers: { Authorization: `Bearer ${token}`, 'Content-Type': 'application/json' },
    data: {},
  }

  switch (method) {
    case 'PUT':
    case 'POST':
      packet.data = JSON.stringify(data)
    case 'GET':
      packet.data = data
  }

  return new Promise((resolve) => {
    axios(packet)
      .then((res) => resolve([res.data, undefined]))
      .catch((err) => resolve([undefined, err]))
  })
}

export { request }
