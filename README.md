> this repo is a fork from [gamba](https://github.com/chikof/gamba)

## akashi_api
simple api for akashi discord bot, serves as database manager.

### routes
> wip table

all routes are authenticated with a bearer token, which is stored in the `BEARER_TOKEN` environment variable, I recommend using a strong token and also changing it often, this token grants all access to the api.

| Route       | Method | Description                                          |
|:------------|--------|------------------------------------------------------|
| /           | GET    | API basic metadata                                   |
| /guilds/:id | GET    | Get information about a guild record in the database |
| /guilds/:id | PATCH  | Update a guild record                                |
| /guilds/:id | DELETE | Delete a guild record                                |
| /guilds     | POST   | Create a guild record                                |
> i am too lazy to put more routes here, check the code for more info
### usage
1. clone the repo
2. install dependencies
3. do whatever idk

### license
idk bro just give credits or sumn