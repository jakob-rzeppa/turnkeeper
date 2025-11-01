# How to use the project

## Development

1. Make sure to install all dependencies in the backend, gm and user with `npm install`.
2. Now you can run the backend, user and gm each with `npm run dev`.

## Deployment

1. Make sure to install all dependencies in the backend, gm and user with `npm install`.
2. Set the .env variables `BACKEND_URL` in user and gm to the right url. 'http://localhost:3000/gm' for the gm and 'http://[ip address]:3000/user' for the users.
3. Now run `npm run build` in all three projects.
4. In the backend run `npm run start`.
5. For the gm and user go into the /dist directory and serve the site (for example using serve https://www.npmjs.com/package/serve).
6. Now you are ready to give the users on their phones the url of the user frontend.

> Make sure your firewall doesn't block their requests.
